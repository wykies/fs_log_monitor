use std::{fs, time::Duration};

use anyhow::{bail, Context};

pub struct Discord {
    url: String,
}

impl Discord {
    const RETRY_ATTEMPTS: u8 = 3;
    const INTERVAL_BETWEEN_RETRY: Duration = Duration::from_secs(30);

    pub fn new() -> anyhow::Result<Self> {
        let filename = "d.data";
        let url_suffix = fs::read_to_string(filename).with_context(|| {
            format!("failed to read discord webhook url suffix from {filename:?}")
        })?;
        let url = format!("https://discord.com/api/webhooks/{url_suffix}");
        Ok(Self { url })
    }

    pub fn send(&self, msg: &str) -> anyhow::Result<()> {
        for i in 0..Self::RETRY_ATTEMPTS {
            // Wait before trying again
            if i > 0 {
                std::thread::sleep(Self::INTERVAL_BETWEEN_RETRY);
            }

            todo!("Send message using blocking reqwest");

            // match self
            //     .rt
            //     .block_on(self.do_send(msg))
            //     .context("failed to send ")
            // {
            //     Ok(()) => return Ok(()),
            //     Err(e) => error!(
            //         "attempt #{} failed to send via discord. Error: {e:?}",
            //         i + 1
            //     ),
            // }
        }
        bail!(
            "failed to send via discord after {} attempts",
            Self::RETRY_ATTEMPTS
        )
    }
}
