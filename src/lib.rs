use arma_rs::{arma, Context, Extension, Group};

mod openshock;

#[arma]
fn init() -> Extension {
    Extension::build()
        .group(
            "ops",
            Group::new()
                .command("shock", shock)
                .command("vibrate", vibrate)
                .command("beep", beep),
        )
        .finish()
}

fn shock(
    ctx: Context,
    shocker_id: String,
    api_token: String,
    intensity: u32,
    duration: u32,
) -> Result<String, String> {
    dispatch(
        ctx,
        shocker_id,
        api_token,
        openshock::Operation::Shock,
        intensity,
        duration,
    )
}

fn vibrate(
    ctx: Context,
    shocker_id: String,
    api_token: String,
    intensity: u32,
    duration: u32,
) -> Result<String, String> {
    dispatch(
        ctx,
        shocker_id,
        api_token,
        openshock::Operation::Vibrate,
        intensity,
        duration,
    )
}

fn beep(
    ctx: Context,
    shocker_id: String,
    api_token: String,
    duration: u32,
) -> Result<String, String> {
    dispatch(
        ctx,
        shocker_id,
        api_token,
        openshock::Operation::Sound,
        0,
        duration,
    )
}

fn dispatch(
    ctx: Context,
    shocker_id: String,
    api_token: String,
    operation: openshock::Operation,
    intensity: u32,
    duration: u32,
) -> Result<String, String> {
    let request = openshock::ControlRequest::new(shocker_id, operation, intensity, duration)?;
    let token = openshock::token_header(&api_token)?;
    // Keep network I/O off Arma's game thread. A queued request is not an API success.
    std::thread::Builder::new()
        .name("openshock-request".into())
        .spawn(move || {
            let result = openshock::send(openshock::CONTROL_URL, token, &request)
                .unwrap_or_else(|error| error);
            let _ = ctx.callback_data("arma3_openshock", operation.callback_name(), Some(result));
        })
        .map_err(|_| "Could not start OpenShock request worker".to_string())?;
    Ok(format!("{} request queued", operation.callback_name()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extension_commands_reject_invalid_arguments_without_network_io() {
        let extension = init().testing();
        for command in ["ops:shock", "ops:vibrate", "ops:beep"] {
            let mut args = vec!["not-a-uuid".to_string(), "test-token".to_string()];
            if command != "ops:beep" {
                args.push("1".into());
            }
            args.push("1".into());
            let (message, code) = extension.call(command, Some(args));
            assert_eq!(code, 9, "{command}: {message}");
            assert!(message.contains("Shocker ID"));
        }
    }
}
