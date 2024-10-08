/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use crate::{error::Error, RefinedSearchConfig, SearchApiResult, SearchUserEnvironment};
use error_support::handle_error;

/// SearchEngineSelector parses the JSON configuration for
/// search engines and returns the applicable engines depending
/// on their region + locale.
#[derive(Default, uniffi::Object)]
pub struct SearchEngineSelector {}

#[uniffi::export]
impl SearchEngineSelector {
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the search configuration from the given string. If the configuration
    /// string is unchanged since the last update, the cached configuration is
    /// reused to avoid unnecessary reprocessing. This helps optimize performance,
    /// particularly during test runs where the same configuration may be used
    /// repeatedly.
    #[handle_error(Error)]
    pub fn set_search_config(&self, _configuration: String) -> SearchApiResult<()> {
        Err(Error::NotImplemented)
    }

    /// Clears the search configuration from memory if it is known that it is
    /// not required for a time, e.g. if the configuration will only be re-filtered
    /// after an app/environment update.
    pub fn clear_search_config(&self) {}

    /// Filters the search configuration with the user's given environment,
    /// and returns the set of engines and parameters that should be presented
    /// to the user.
    #[handle_error(Error)]
    pub fn filter_engine_configuration(
        &self,
        _user_environment: SearchUserEnvironment,
    ) -> SearchApiResult<RefinedSearchConfig> {
        Err(Error::NotImplemented)
    }
}

#[cfg(test)]
mod tests {
    use super::{SearchEngineSelector, SearchUserEnvironment};
    use crate::types::*;

    #[test]
    fn test_filter_engine_config_throws() {
        let selector = SearchEngineSelector::new();

        let result = selector.filter_engine_configuration(SearchUserEnvironment {
            locale: "fi".into(),
            region: "FR".into(),
            update_channel: SearchUpdateChannel::Default,
            distribution_id: String::new(),
            experiment: String::new(),
            app_name: SearchApplicationName::Firefox,
            version: String::new(),
        });

        assert!(result.is_err());
    }
}
