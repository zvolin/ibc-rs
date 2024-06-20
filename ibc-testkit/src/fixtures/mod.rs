pub mod applications;
pub mod clients;
pub mod core;
use alloc::fmt::Debug;

use ibc::core::handler::types::error::ContextError;
use ibc::core::primitives::prelude::*;

use crate::hosts::TestHost;
use crate::testapp::ibc::core::types::DefaultIbcStore;
pub enum Expect {
    Success,
    Failure(Option<ContextError>),
}

#[derive(Debug)]
pub struct Fixture<H: TestHost, M: Debug> {
    pub ctx: DefaultIbcStore<H>,
    pub msg: M,
}

impl<H: TestHost, M: Debug> Fixture<H, M> {
    pub fn generate_error_msg(
        &self,
        expect: &Expect,
        process: &str,
        res: &Result<(), ContextError>,
    ) -> String {
        let base_error = match expect {
            Expect::Success => "step failed!",
            Expect::Failure(_) => "step passed but was supposed to fail!",
        };
        format!(
            "{process} {base_error} /n {res:?} /n {:?} /n {:?}",
            &self.msg, &self.ctx
        )
    }
}
