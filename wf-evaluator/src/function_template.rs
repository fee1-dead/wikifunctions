extern crate wfrt;

#[no_mangle]
pub extern "C" fn evaluate_entrypoint(input: wfrt::ffi::Bytes<'_>) -> wfrt::ffi::OwnedBytes {{
    #[allow(non_snake_case)]
    // function begin
    {code}
    // function end

    fn evaluate_inner(bytes: wfrt::ffi::Bytes<'_>) -> Result<wfrt::Value, String> {{
        let values = wfrt::bytes_to_values(bytes).map_err(|e| e.to_string())?;
        let result: Result<Result<wfrt::Value, String>, _> = std::panic::catch_unwind(move || {{
            let mut values = values.into_iter();
            #[allow(unused)]
            fn get_next<T: TryFrom<wfrt::Value>>(values: &mut std::vec::IntoIter<wfrt::Value>) -> Result<T, String>
            where
                T::Error: ToString,
            {{
                values.next().ok_or_else(|| "out of values".to_owned()).and_then(|v| T::try_from(v).map_err(|e| e.to_string()))
            }}
            let ret = {fn_name}({fn_args});
            Ok(wfrt::IntoValue::into_value(ret))
        }});

        // TODO recover panic message
        result.map_err(|_| "function panicked".to_owned())?
    }}

    let result = evaluate_inner(input);

    wfrt::ffi::OwnedBytes::from_vec(wfrt::to_stdvec(&result).expect("serialization cannot fail"))
}}