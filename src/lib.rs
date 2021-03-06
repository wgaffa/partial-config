use std::marker::PhantomData;

use partial_functional::{Monoid, Semigroup};

pub trait ConfigBuilder {
    type Target;

    fn build(self) -> Self::Target;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Build;

#[derive(Debug, Default, Clone, Copy)]
pub struct Run;

#[derive(Debug, Clone, Copy)]
enum Selection<M, A> {
    Build(M),
    Run(A),
}

impl<M: Default, A> Default for Selection<M, A> {
    fn default() -> Self {
        Self::Build(M::default())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Select<T, M, A> {
    inner: Selection<M, A>,
    _phantom_data: PhantomData<T>,
}

impl<M: Monoid, A> Default for Select<Build, M, A> {
    fn default() -> Self {
        Self {
            inner: Selection::Build(Monoid::empty()),
            _phantom_data: PhantomData,
        }
    }
}

impl<M, A> Select<Build, M, A> {
    pub fn get(&self) -> &M {
        if let Selection::Build(ref x) = self.inner {
            x
        } else {
            panic!("Select in wrong state")
        }
    }
}

impl<M, A> Select<Run, M, A> {
    pub fn value(&self) -> &A {
        if let Selection::Run(ref x) = self.inner {
            x
        } else {
            panic!("Select in wrong state")
        }
    }

    pub fn get(self) -> A {
        if let Selection::Run(x) = self.inner {
            x
        } else {
            panic!("Select in wrong state")
        }
    }
}

impl<M, A> Select<Run, M, A>
where
    A: Default,
{
    pub fn take(&mut self) -> A {
        if let Selection::Run(ref mut x) = self.inner {
            std::mem::take(x)
        } else {
            panic!("Select in wrong state")
        }
    }
}

impl<M, A> From<A> for Select<Build, M, A>
where
    M: Monoid,
    A: Into<M>,
{
    fn from(value: A) -> Self {
        Self {
            inner: Selection::Build(value.into()),
            _phantom_data: PhantomData,
        }
    }
}

impl<M: Monoid, A> From<A> for Select<Run, M, A> {
    fn from(val: A) -> Self {
        Self {
            inner: Selection::Run(val),
            _phantom_data: PhantomData,
        }
    }
}

impl<M, A> From<Select<Run, M, A>> for Select<Build, M, A>
where
    M: Monoid,
    A: Into<M>,
{
    fn from(value: Select<Run, M, A>) -> Self {
        let value = match value.inner {
            Selection::Build(_) => panic!("Select in wrong state"),
            Selection::Run(x) => x,
        };

        Self {
            inner: Selection::Build(value.into()),
            _phantom_data: PhantomData,
        }
    }
}

impl<M, A> From<Select<Build, M, A>> for Select<Run, M, A>
where
    M: Monoid + Into<A>,
{
    fn from(value: Select<Build, M, A>) -> Self {
        let value = match value.inner {
            Selection::Run(_) => panic!("Select in wrong state"),
            Selection::Build(x) => x,
        };

        Self {
            inner: Selection::Run(value.into()),
            _phantom_data: PhantomData,
        }
    }
}

impl<M: Semigroup, A> Semigroup for Select<Build, M, A> {
    fn combine(self, rhs: Self) -> Self {
        Self {
            inner: match (self.inner, rhs.inner) {
                (Selection::Build(left), Selection::Build(right)) => {
                    Selection::Build(left.combine(right))
                }
                _ => panic!("Select Build was in a wrong state to combine"),
            },
            _phantom_data: PhantomData,
        }
    }
}

#[macro_export]
macro_rules! config {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident
        {
            $(
                $(#[$field_meta:meta])*
                $field_vis:vis $field_name:ident : { $m:ty , $a:ty }
            ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        $vis struct $name<T> {
            $(
                $(#[$field_meta])*
                $field_vis $field_name: $crate::Select<T, $m, $a>,
            )*
        }

        impl Semigroup for $name<$crate::Build> {
            fn combine(self, rhs: Self) -> Self {
                Self {
                    $(
                        $field_name: self.$field_name.combine(rhs.$field_name),
                    )*
                }
            }
        }

        impl Default for $name<$crate::Build> {
            fn default() -> Self {
                Self {
                    $(
                        $field_name: Default::default(),
                    )*
                }
            }
        }
    };
}

#[macro_export]
macro_rules! config_builder {
    ($t:ident { $($field:ident => $e:expr),* $(,)? }) => {
        impl $t<Build> {
            pub fn build(self) -> $t<Run> {
                $t {
                    $(
                        $field: {
                            let tmp = $e;
                            tmp(self.$field.inner)
                        },
                    )*
                }
            }
        }
    };
}

impl Semigroup for Build {
    fn combine(self, _rhs: Self) -> Self {
        self
    }
}
