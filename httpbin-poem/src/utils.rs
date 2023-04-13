use poem::Route;

pub(crate) trait RouteExt {
    fn attach(self, f: impl Fn(Self) -> Self) -> Self
    where
        Self: Sized,
    {
        f(self)
    }
}

impl RouteExt for Route {}
