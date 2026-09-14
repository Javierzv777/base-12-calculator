//! The calculator's function registry -- the extension point PLAN.md
//! Section 6 asks for, in the shape the rest of Phase 6 needs.
//!
//! Adding a function to the calculator means implementing `DozFunction`
//! and registering it. Nothing else changes: the lexer already produces
//! identifiers, the parser already produces `Expr::Call`, the evaluator
//! already dispatches through here, and every interface (REPL, one-shot
//! CLI, GUI) lists what it finds in the registry instead of hard-coding a
//! keypad. That is what makes `sin`/`cos` a registration rather than a
//! project.
//!
//! # Why a function declares two implementations
//!
//! The calculator has an exact mode (rationals) and an approximate one
//! (fixed point). `abs` and `floor` are exact in both; `sin` is not exact
//! anywhere. Rather than force every function to pick a lane, a
//! `DozFunction` must implement the fixed-point case and *may* override
//! the rational one. The default rational implementation converts its
//! arguments to fixed point, calls the fixed-point implementation, and
//! lifts the result back to an exact rational of that truncated value --
//! which is honest only because the result is also flagged as
//! approximate, so the display can say so (`~` in the interfaces) instead
//! of printing a twelve-digit fraction as if it were the true value of
//! `sin(1)`.

use crate::EvalError;
use dz_gates::{DozGates, DynGates};
use dz_num::{DzFixed, DzRational};
use std::collections::HashMap;

/// Working precision, in fractional dozenal digits, for functions that
/// cannot be exact. Interfaces expose it (`--scale`, `:scale`) so a
/// research session can trade digits against gate count.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FnContext {
    pub scale: usize,
}

impl Default for FnContext {
    fn default() -> FnContext {
        FnContext { scale: dz_sci::DEFAULT_SCI_SCALE }
    }
}

/// A result plus whether it is the exact value or a truncation of it.
#[derive(Clone, Debug)]
pub struct FnValue<T> {
    pub value: T,
    pub exact: bool,
}

impl<T> FnValue<T> {
    pub fn exact(value: T) -> FnValue<T> {
        FnValue { value, exact: true }
    }
    pub fn approximate(value: T) -> FnValue<T> {
        FnValue { value, exact: false }
    }
}

/// How many arguments a function accepts. A closed range rather than a
/// single number, so `log(x)` (base twelve) and `log(b, x)` can be one
/// entry instead of two names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Arity {
    pub min: usize,
    pub max: usize,
}

impl Arity {
    pub const fn exact(n: usize) -> Arity {
        Arity { min: n, max: n }
    }
    pub const fn range(min: usize, max: usize) -> Arity {
        Arity { min, max }
    }
    pub fn accepts(&self, n: usize) -> bool {
        n >= self.min && n <= self.max
    }
}

/// Grouping for interfaces that lay functions out (a GUI keypad, `:funcs`
/// in the REPL, `--help` in the one-shot CLI). Purely presentational --
/// the evaluator never looks at it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Category {
    Constant,
    Rounding,
    Root,
    Exponential,
    Trigonometry,
    /// Functions that are registered but not meant for a keypad (see
    /// `FnInfo::cli_only`). Currently only `gamma`.
    Special,
}

impl Category {
    pub fn label(&self) -> &'static str {
        match self {
            Category::Constant => "constant",
            Category::Rounding => "rounding",
            Category::Root => "root",
            Category::Exponential => "exponential",
            Category::Trigonometry => "trigonometry",
            Category::Special => "special",
        }
    }
}

/// Everything an interface needs to render a function without knowing what
/// it computes.
#[derive(Clone, Copy, Debug)]
pub struct FnInfo {
    pub name: &'static str,
    pub arity: Arity,
    pub category: Category,
    pub description: &'static str,
    /// Whether results are exact in every case (`abs`) or an approximation
    /// at the working scale (`sin`).
    pub always_exact: bool,
    /// Whether this function should be hidden from interfaces that render
    /// the catalog as a fixed keypad (the GUI's scientific pad). The
    /// registry is still the single point of extension -- this is a
    /// presentational flag on `FnInfo`, not a second table: `dz-cli` reads
    /// the catalog unfiltered, and `app/src-tauri::commands::list_functions`
    /// is the one place that filters it out before sending it to the
    /// frontend (see `gamma` in `dz-eval::builtins`, the function this flag
    /// was added for).
    pub cli_only: bool,
}

/// Implement this and register it; see the module documentation.
pub trait DozFunction: Send + Sync {
    fn name(&self) -> &'static str;
    fn arity(&self) -> Arity;
    fn category(&self) -> Category;
    fn description(&self) -> &'static str;

    /// Whether this function is exact for every input. `sqrt` says `false`
    /// even though `sqrt(9)` is exact, because that is decided per call --
    /// the per-call answer is the `exact` field of `FnValue`.
    fn always_exact(&self) -> bool {
        false
    }

    /// Whether this function is excluded from interfaces that render the
    /// catalog as a fixed keypad (see `FnInfo::cli_only`). Defaults to
    /// visible everywhere, matching every function registered before this
    /// flag existed.
    fn cli_only(&self) -> bool {
        false
    }

    /// The fixed-point implementation. Required.
    fn apply_fixed(
        &self,
        g: &dyn DozGates,
        args: &[DzFixed],
        ctx: &FnContext,
    ) -> Result<FnValue<DzFixed>, EvalError>;

    /// The exact-rational implementation. The default routes through
    /// `apply_fixed` at the working scale and reports the outcome's own
    /// exactness flag; exact functions override it to stay exact.
    fn apply_rational(
        &self,
        g: &dyn DozGates,
        args: &[DzRational],
        ctx: &FnContext,
    ) -> Result<FnValue<DzRational>, EvalError> {
        let gates = DynGates(g);
        let working = ctx.scale + dz_sci::GUARD_DIGITS;
        let fixed_args: Vec<DzFixed> = args.iter().map(|a| a.to_fixed(&gates, working)).collect();
        let out = self.apply_fixed(g, &fixed_args, ctx)?;
        Ok(FnValue {
            value: DzRational::from_fixed(&gates, &out.value),
            exact: out.exact,
        })
    }

    fn info(&self) -> FnInfo {
        FnInfo {
            name: self.name(),
            arity: self.arity(),
            category: self.category(),
            description: self.description(),
            always_exact: self.always_exact(),
            cli_only: self.cli_only(),
        }
    }
}

/// A set of named functions. Registration order is preserved so interfaces
/// can show a stable, hand-ordered catalog instead of a hash order.
#[derive(Default)]
pub struct FnRegistry {
    functions: HashMap<&'static str, Box<dyn DozFunction>>,
    order: Vec<&'static str>,
}

impl FnRegistry {
    /// Empty -- an interface that wants only its own functions.
    pub fn new() -> FnRegistry {
        FnRegistry { functions: HashMap::new(), order: Vec::new() }
    }

    /// Every built-in function (`crate::builtins`).
    pub fn with_builtins() -> FnRegistry {
        let mut registry = FnRegistry::new();
        crate::builtins::register_all(&mut registry);
        registry
    }

    pub fn register(&mut self, function: Box<dyn DozFunction>) {
        let name = function.name();
        if !self.functions.contains_key(name) {
            self.order.push(name);
        }
        self.functions.insert(name, function);
    }

    pub fn get(&self, name: &str) -> Option<&dyn DozFunction> {
        self.functions.get(name).map(|f| f.as_ref())
    }

    pub fn contains(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }

    /// The catalog, in registration order: what a keypad or a `:funcs`
    /// listing renders.
    pub fn catalog(&self) -> Vec<FnInfo> {
        self.order
            .iter()
            .filter_map(|name| self.functions.get(name))
            .map(|f| f.info())
            .collect()
    }

    /// Check the argument count before dispatch, so every function's
    /// implementation can index its arguments without re-validating.
    pub fn check_arity(&self, name: &str, got: usize) -> Result<&dyn DozFunction, EvalError> {
        let function = self
            .get(name)
            .ok_or_else(|| EvalError::UnknownFunction(name.to_string()))?;
        let arity = function.arity();
        if !arity.accepts(got) {
            return Err(EvalError::WrongArity { expected: arity.min, got });
        }
        Ok(function)
    }
}
