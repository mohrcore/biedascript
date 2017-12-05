#[macro_export]

macro_rules! concat_idents {
    ($($e:ident),*) => { ... };
}

macro_rules! classic_enum_r {

    { $(access: $acc: ident:)* enum $enum_name: ident is $enum_type: ty { $($name: ident = $val: tt),*, } on error: $err_val: tt} => {

        $($acc)* enum $enum_name {
            ERROR,
            $($name),*
        }

        impl $enum_name {
            $($acc)* fn value(&self) -> $enum_type {

                use self::$enum_name::*;

                match *self {
                    $($name => $val),*,
                    ERROR => $err_val,
                }
            }

            $($acc)* fn from_value(v: $enum_type) -> Self {

                use self::$enum_name::*;

                $(if v == $val { return $name; });*
                ERROR
            }
        }

        impl ::std::string::ToString for $enum_name {
            fn to_string(&self) -> String {

                use self::$enum_name::*;

                match *self {
                    $($name => stringify!($name).to_string()),*,
                    ERROR => "ERROR".to_string(),
                }
            }
        }

        impl ::std::str::FromStr for $enum_name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, String> {

                use self::$enum_name::*;

                match s {
                    $(stringify!($name) => Ok($name)),*,
                    _ => Err(String::from("Unknown name")),
                }
            }
        }

        impl ::std::fmt::Debug for $enum_name {

            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                
                write!(f, "{}", self.to_string())
            }
        }
    }
}