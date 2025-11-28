#[macro_export]
macro_rules! bitflags_union {
    ($ty:ty [$flag:ident]) => {
        <$ty>::$flag
    };
    ($ty:ty [$($flag:ident),+]) => {
        <$ty>::from_bits_truncate(0 $( | <$ty>::$flag.bits() )+)
    };
}

#[macro_export]
macro_rules! oneline_dbg {
    () => {
        eprintln!("[{}:{}]", file!(), line!())
    };
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                eprintln!("[{}:{}] {} = {:?}",
                    file!(), line!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($(oneline_dbg!($val)),+,)
    };
}

#[macro_export]
macro_rules! for_input {
    ($iter:ident, |$ele:ident| $body:tt) => {
        let mut m_iter = $iter;
        while let Some($ele) = Iterator::next(&mut m_iter) {
            let $ele = $ele?;
            $body;
        }
    };
}

#[macro_export]
macro_rules! for_ok {
    ($ele:ident in $iter:ident { $body:tt }) => {
        for $ele in $iter {
            $ele = $ele?;

            $body
        }
    };
}

#[macro_export]
macro_rules! some_or_continue {
    ($e:expr) => {{
        let __res: Option<_> = $e;
        match __res {
            Some(val) => val,
            None => continue,
        }
    }};
}

#[macro_export]
macro_rules! some_or_break {
    ($e:expr $(=> $life:tt)?) => {
        {
            let __res: Option<_> = $e;
            match __res {
                Some(val) => val,
                None => break $($life)?,
            }
        }
    }
}

#[macro_export]
macro_rules! ok_or_continue {
    ($e:expr) => {{
        let __res: Result<_, _> = $e;
        match $e {
            Ok(val) => val,
            Err(_) => continue,
        }
    }};
}
