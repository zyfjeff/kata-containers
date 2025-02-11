// Copyright (c) 2022 Intel Corporation
//
// SPDX-License-Identifier: Apache-2.0
//

use crate::types::*;
#[cfg(target_arch = "powerpc64le")]
pub use arch_specific::*;

mod arch_specific {
    use crate::check;
    use crate::utils;
    use anyhow::Result;

    pub const ARCH_CPU_VENDOR_FIELD: &str = "";
    pub const ARCH_CPU_MODEL_FIELD: &str = "model";

    pub fn check() -> Result<()> {
        unimplemented!("Check not implemented in powerpc64le");
    }

    pub fn get_checks() -> Option<&'static [CheckItem<'static>]> {
        None
    }

    const PEF_SYS_FIRMWARE_DIR: &str = "/sys/firmware/ultravisor/";

    pub fn get_cpu_details() -> Result<(String, String)> {
        utils::get_generic_cpu_details(check::PROC_CPUINFO)

        // TODO: In case of error from get_generic_cpu_details, implement functionality
        // to get cpu details specific to powerpc architecture similar
        // to the goloang implementation of function getCPUDetails()
    }

    pub fn host_is_vmcontainer_capable() -> Result<bool> {
        // TODO: Not implemented
        Ok(true)
    }

    pub fn available_guest_protection() -> Result<check::GuestProtection, check::ProtectionError> {
        if !Uid::effective().is_root() {
            return Err(check::ProtectionError::NoPerms);
        }

        let metadata = fs::metadata(PEF_SYS_FIRMWARE_DIR);
        if metadata.is_ok() && metadata.unwrap().is_dir() {
            Ok(check::GuestProtection::Pef)
        }

        Ok(check::GuestProtection::NoProtection)
    }
}
