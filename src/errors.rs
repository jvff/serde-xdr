macro_rules! bail {
    ($error:expr $(,)*) => {{
        let _ = Err($error)?;
        unreachable!();
    }};
}

macro_rules! ensure {
    ($condition:expr , $error:expr $(,)*) => {
        if !$condition {
            bail!($error)
        }
    };
}
