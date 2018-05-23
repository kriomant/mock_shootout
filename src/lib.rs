#![feature(proc_macro)]     // Needed by mockers and mock_derive
#![feature(extern_prelude)] // Needed by double
#![feature(proc_macro_mod)] // Needed by galvanic_mock
#![feature(range_contains)] // Used by our code

#![allow(dead_code)]

extern crate galvanic_assert;
extern crate galvanic_mock;
#[macro_use] extern crate double;
extern crate mock_derive;
extern crate mockers;
extern crate mockers_derive;
extern crate simulacrum;
extern crate simulacrum_user;

#[allow(unused)]
macro_rules! test {
    ( $self:ident) => {
        #[test] fn associated_types() { $self::associated_types() }
        #[test] fn checkpoint() { $self::checkpoint() }
        #[test] fn consume() { $self::consume() }
        #[test] fn derive() { $self::derive() }
        #[test] fn external_trait() { $self::external_trait() }
        #[test] fn fallback() { $self::fallback() }
        #[test] fn foreign() { $self::foreign() }
        #[test] fn generic_method() { $self::generic_method() }
        #[test] fn generic_trait() { $self::generic_trait() }
        #[test] fn inherited_trait() { $self::inherited_trait() }
        #[test] fn many_args() { $self::many_args() }
        #[test] fn match_and() { $self::match_and() }
        #[test] fn match_constant() { $self::match_constant() }
        #[test] fn match_method() { $self::match_method() }
        #[test] fn match_operator() { $self::match_operator() }
        #[test] fn match_or() { $self::match_or() }
        #[test] fn match_pattern() { $self::match_pattern() }
        #[test] fn match_range() { $self::match_range() }
        #[test] fn match_wildcard() { $self::match_wildcard() }
        #[test] fn multi_trait() { $self::multi_trait() }
        #[test] fn return_call() { $self::return_call() }
        #[test] fn return_call_with_args() { $self::return_call_with_args() }
        #[test] fn return_constant() { $self::return_constant() }
        #[test] fn return_default() { $self::return_default() }
        #[test] fn return_lifetime() { $self::return_lifetime() }
        #[test] fn return_owned() { $self::return_owned() }
        #[should_panic(expected="Panic")] #[test] fn return_panic() {
            $self::return_panic()
        }
        #[test] fn return_parameters() { $self::return_parameters() }
        #[test] fn sequence() { $self::sequence() }
        #[test] fn times_once() { $self::times_once() }
        #[test] fn times_any() { $self::times_any() }
        #[test] fn times_never() { $self::times_never() }
        #[test] fn times_range() { $self::times_range() }
    }
}

mod t_double;
mod t_galvanic_mock;
mod t_mock_derive;
mod t_mockers;
mod t_simulacrum;

pub trait TestSuite {
    // Core features.  These are the essential features that cannot be
    // implemented by the user
    /// A mocked `Trait` can have associated types
    fn associated_types();
    /// A mock can set a barrier.  All expectations defined before the barrier must
    /// be satisified before, and all expectations defined after the barrier
    /// must be satisfied after.
    fn checkpoint();
    /// A mock method can consume `self`
    fn consume();
    /// A Mock can be defined for a `Trait` in an external crate;
    fn external_trait();
    /// A mock object can fallback to the real object's behavior
    fn fallback();
    /// Can mock foreign functions
    fn foreign();
    /// A `Trait` with a generic method can be mocked.
    fn generic_method();
    /// A generic `Trait` can be mocked.
    fn generic_trait();
    /// An inherited `Trait` can be mocked.
    fn inherited_trait();
    /// A method with many arguments can be mocked
    fn many_args();
    /// A method call can match an argument by an arbitrary method
    fn match_method();
    /// A Mock can be defined that implements multiple `Trait`s.
    fn multi_trait();
    /// A mock method can return the result of a function that depends on its
    /// arguments.
    fn return_call_with_args();
    /// A mock method can return a reference with a non-`'static` lifetime
    fn return_lifetime();
    /// A mock method can return an owned object that is neither `Copy` nor
    /// `Clone`
    fn return_owned();
    /// A mock method can mutate its parameters when supplied by mutable
    /// reference.
    fn return_parameters();
    /// A scenario can expect calls to multiple methods in a specified order
    fn sequence();
    /// An expectation can assert that it's called a variable number of times
    fn times_range();

    // Convenience features.  These are nice-to-have, but their absence doesn't
    // fundamentally limit the library's capabilities.  The user can either
    // implement these himself, or make do with their lack.
    /// A Mock can be `Derive`d from a `Trait`
    fn derive();
    /// A method call can match both of two conditions
    fn match_and();
    /// A method call can match a constant value
    fn match_constant();
    /// A method call can match an argument by common operators
    fn match_operator();
    /// A method call can match either of two conditions
    fn match_or();
    /// A method call can match a pattern
    fn match_pattern();
    /// A method call can match a range of values
    fn match_range();
    /// A method call can match any argument at all
    fn match_wildcard();
    /// A mock method can return the result of a function
    fn return_call();
    /// A mock method can return a constant value
    fn return_constant();
    /// A mock method can return the type's default value
    fn return_default();
    /// A mock method can panic, rather than return
    fn return_panic();
    /// An expectation can assert that it's called only once
    fn times_once();
    /// An expectation can be called any number of times
    fn times_any();
    /// An expectation can assert that it's never called
    fn times_never();
}

