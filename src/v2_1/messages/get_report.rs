use crate::v2_1::datatypes::{ComponentVariableType, CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::{ComponentCriterionEnumType, GenericDeviceModelStatusEnumType};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for the GetReport request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetReportRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))]
    #[validate(nested)]
    pub component_variable: Option<Vec<ComponentVariableType>>,

    /// The Id of the request.
    #[validate(range(min = 0))]
    pub request_id: i32,

    /// This field contains criteria for components for which a report is requested
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 4))]
    pub component_criteria: Option<Vec<ComponentCriterionEnumType>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl GetReportRequest {
    /// Creates a new instance of the struct.
    ///
    /// * `request_id` - The Id of the request.
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(request_id: i32) -> Self {
        Self {
            component_variable: None,
            request_id,
            component_criteria: None,
            custom_data: None,
        }
    }

    /// Sets the component_variable field.
    ///
    /// * `component_variable` - The component_variable field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_component_variable(&mut self, component_variable: Option<Vec<ComponentVariableType>>) -> &mut Self {
        self.component_variable = component_variable;
        self
    }

    /// Sets the request_id field.
    ///
    /// * `request_id` - The Id of the request.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_request_id(&mut self, request_id: i32) -> &mut Self {
        self.request_id = request_id;
        self
    }

    /// Sets the component_criteria field.
    ///
    /// * `component_criteria` - This field contains criteria for components for which a report is requested
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_component_criteria(&mut self, component_criteria: Option<Vec<ComponentCriterionEnumType>>) -> &mut Self {
        self.component_criteria = component_criteria;
        self
    }

    /// Sets the custom_data field.
    ///
    /// * `custom_data` - The custom_data field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets a reference to the component_variable field.
    ///
    /// # Returns
    ///
    /// The component_variable field
    pub fn get_component_variable(&self) -> Option<&Vec<ComponentVariableType>> {
        self.component_variable.as_ref()
    }

    /// Gets a reference to the request_id field.
    ///
    /// # Returns
    ///
    /// The Id of the request.
    pub fn get_request_id(&self) -> &i32 {
        &self.request_id
    }

    /// Gets a reference to the component_criteria field.
    ///
    /// # Returns
    ///
    /// This field contains criteria for components for which a report is requested
    pub fn get_component_criteria(&self) -> Option<&Vec<ComponentCriterionEnumType>> {
        self.component_criteria.as_ref()
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the component_variable field and returns self for builder pattern.
    ///
    /// * `component_variable` - The component_variable field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_component_variable(mut self, component_variable: Vec<ComponentVariableType>) -> Self {
        self.component_variable = Some(component_variable);
        self
    }

    /// Sets the component_criteria field and returns self for builder pattern.
    ///
    /// * `component_criteria` - This field contains criteria for components for which a report is requested
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_component_criteria(mut self, component_criteria: Vec<ComponentCriterionEnumType>) -> Self {
        self.component_criteria = Some(component_criteria);
        self
    }

    /// Sets the custom_data field and returns self for builder pattern.
    ///
    /// * `custom_data` - The custom_data field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

}

/// Response body for the GetReport response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetReportResponse {
    pub status: GenericDeviceModelStatusEnumType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl GetReportResponse {
    /// Creates a new instance of the struct.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A new instance of the struct with required fields set and optional fields as None.
    pub fn new(status: GenericDeviceModelStatusEnumType) -> Self {
        Self {
            status,
            status_info: None,
            custom_data: None,
        }
    }

    /// Sets the status field.
    ///
    /// * `status` - The status field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_status(&mut self, status: GenericDeviceModelStatusEnumType) -> &mut Self {
        self.status = status;
        self
    }

    /// Sets the status_info field.
    ///
    /// * `status_info` - The status_info field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_status_info(&mut self, status_info: Option<StatusInfoType>) -> &mut Self {
        self.status_info = status_info;
        self
    }

    /// Sets the custom_data field.
    ///
    /// * `custom_data` - The custom_data field
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets a reference to the status field.
    ///
    /// # Returns
    ///
    /// The status field
    pub fn get_status(&self) -> &GenericDeviceModelStatusEnumType {
        &self.status
    }

    /// Gets a reference to the status_info field.
    ///
    /// # Returns
    ///
    /// The status_info field
    pub fn get_status_info(&self) -> Option<&StatusInfoType> {
        self.status_info.as_ref()
    }

    /// Gets a reference to the custom_data field.
    ///
    /// # Returns
    ///
    /// The custom_data field
    pub fn get_custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the status_info field and returns self for builder pattern.
    ///
    /// * `status_info` - The status_info field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_status_info(mut self, status_info: StatusInfoType) -> Self {
        self.status_info = Some(status_info);
        self
    }

    /// Sets the custom_data field and returns self for builder pattern.
    ///
    /// * `custom_data` - The custom_data field
    ///
    /// # Returns
    ///
    /// Self with the field set.
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2_1::datatypes::ComponentVariableType;
    use serde_json;

    fn create_test_custom_data() -> CustomDataType {
        CustomDataType::new("TestVendor".to_string())
    }

    fn create_test_status_info() -> StatusInfoType {
        StatusInfoType::new("TestReason".to_string())
    }

    fn create_test_component_variable() -> ComponentVariableType {
        ComponentVariableType::new(
            crate::v2_1::datatypes::ComponentType::new("TestComponent".to_string())
        ).with_variable(
            crate::v2_1::datatypes::VariableType::new("TestVariable".to_string())
        )
    }

    // Tests for GetReportRequest

    #[test]
    fn test_get_report_request_new() {
        let request = GetReportRequest::new(123);

        assert_eq!(request.request_id, 123);
        assert_eq!(request.component_criteria, None);
        assert_eq!(request.component_variable, None);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_get_report_request_with_component_criteria() {
        let criteria = vec![ComponentCriterionEnumType::Active];
        let request = GetReportRequest::new(456)
            .with_component_criteria(criteria.clone());

        assert_eq!(request.request_id, 456);
        assert_eq!(request.component_criteria, Some(criteria));
        assert_eq!(request.component_variable, None);
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_get_report_request_with_component_variable() {
        let component_var = vec![create_test_component_variable()];
        let request = GetReportRequest::new(789)
            .with_component_variable(component_var.clone());

        assert_eq!(request.request_id, 789);
        assert_eq!(request.component_criteria, None);
        assert_eq!(request.component_variable, Some(component_var));
        assert_eq!(request.custom_data, None);
    }

    #[test]
    fn test_get_report_request_with_custom_data() {
        let custom_data = create_test_custom_data();
        let request = GetReportRequest::new(999)
            .with_custom_data(custom_data.clone());

        assert_eq!(request.request_id, 999);
        assert_eq!(request.component_criteria, None);
        assert_eq!(request.component_variable, None);
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_get_report_request_setters() {
        let criteria = vec![ComponentCriterionEnumType::Available];
        let component_var = vec![create_test_component_variable()];
        let custom_data = create_test_custom_data();

        let mut request = GetReportRequest::new(100);
        request.set_request_id(200);
        request.set_component_criteria(Some(criteria.clone()));
        request.set_component_variable(Some(component_var.clone()));
        request.set_custom_data(Some(custom_data.clone()));

        assert_eq!(request.request_id, 200);
        assert_eq!(request.component_criteria, Some(criteria));
        assert_eq!(request.component_variable, Some(component_var));
        assert_eq!(request.custom_data, Some(custom_data));
    }

    #[test]
    fn test_get_report_request_getters() {
        let criteria = vec![ComponentCriterionEnumType::Enabled];
        let component_var = vec![create_test_component_variable()];
        let custom_data = create_test_custom_data();
        let request = GetReportRequest::new(555)
            .with_component_criteria(criteria.clone())
            .with_component_variable(component_var.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(request.get_request_id(), &555);
        assert_eq!(request.get_component_criteria(), Some(&criteria));
        assert_eq!(request.get_component_variable(), Some(&component_var));
        assert_eq!(request.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_get_report_request_serialization() {
        let request = GetReportRequest::new(123);

        let json = serde_json::to_string(&request).unwrap();
        let parsed: GetReportRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request, parsed);
    }

    #[test]
    fn test_get_report_request_validation() {
        let request = GetReportRequest::new(100);

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_get_report_request_validation_negative_request_id() {
        let mut request = GetReportRequest::new(100);
        request.set_request_id(-1);

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_get_report_request_validation_empty_component_criteria() {
        let mut request = GetReportRequest::new(100);
        request.set_component_criteria(Some(vec![])); // Empty list should fail validation

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_get_report_request_validation_empty_component_variable() {
        let mut request = GetReportRequest::new(100);
        request.set_component_variable(Some(vec![])); // Empty list should fail validation

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_get_report_request_all_component_criteria() {
        let criteria_types = vec![
            ComponentCriterionEnumType::Active,
            ComponentCriterionEnumType::Available,
            ComponentCriterionEnumType::Enabled,
            ComponentCriterionEnumType::Problem,
        ];

        for criterion in criteria_types {
            let request = GetReportRequest::new(123)
                .with_component_criteria(vec![criterion.clone()]);
            assert_eq!(request.component_criteria, Some(vec![criterion]));
            assert!(request.validate().is_ok());
        }
    }

    #[test]
    fn test_get_report_request_json_round_trip() {
        let criteria = vec![
            ComponentCriterionEnumType::Active,
            ComponentCriterionEnumType::Available,
        ];
        let component_var = vec![create_test_component_variable()];
        let custom_data = create_test_custom_data();
        let request = GetReportRequest::new(777)
            .with_component_criteria(criteria)
            .with_component_variable(component_var)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&request).unwrap();
        let parsed: GetReportRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request, parsed);
        assert!(parsed.validate().is_ok());
    }

    // Tests for GetReportResponse

    #[test]
    fn test_get_report_response_new() {
        let response = GetReportResponse::new(GenericDeviceModelStatusEnumType::Accepted);

        assert_eq!(response.status, GenericDeviceModelStatusEnumType::Accepted);
        assert_eq!(response.status_info, None);
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_get_report_response_with_status_info() {
        let status_info = create_test_status_info();
        let response = GetReportResponse::new(GenericDeviceModelStatusEnumType::Rejected)
            .with_status_info(status_info.clone());

        assert_eq!(response.status, GenericDeviceModelStatusEnumType::Rejected);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.custom_data, None);
    }

    #[test]
    fn test_get_report_response_with_custom_data() {
        let custom_data = create_test_custom_data();
        let response = GetReportResponse::new(GenericDeviceModelStatusEnumType::NotSupported)
            .with_custom_data(custom_data.clone());

        assert_eq!(response.status, GenericDeviceModelStatusEnumType::NotSupported);
        assert_eq!(response.status_info, None);
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_get_report_response_setters() {
        let status_info = create_test_status_info();
        let custom_data = create_test_custom_data();

        let mut response = GetReportResponse::new(GenericDeviceModelStatusEnumType::Accepted);
        response.set_status(GenericDeviceModelStatusEnumType::EmptyResultSet);
        response.set_status_info(Some(status_info.clone()));
        response.set_custom_data(Some(custom_data.clone()));

        assert_eq!(response.status, GenericDeviceModelStatusEnumType::EmptyResultSet);
        assert_eq!(response.status_info, Some(status_info));
        assert_eq!(response.custom_data, Some(custom_data));
    }

    #[test]
    fn test_get_report_response_getters() {
        let status_info = create_test_status_info();
        let custom_data = create_test_custom_data();
        let response = GetReportResponse::new(GenericDeviceModelStatusEnumType::Accepted)
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(response.get_status(), &GenericDeviceModelStatusEnumType::Accepted);
        assert_eq!(response.get_status_info(), Some(&status_info));
        assert_eq!(response.get_custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_get_report_response_serialization() {
        let response = GetReportResponse::new(GenericDeviceModelStatusEnumType::Accepted);

        let json = serde_json::to_string(&response).unwrap();
        let parsed: GetReportResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response, parsed);
    }

    #[test]
    fn test_get_report_response_validation() {
        let response = GetReportResponse::new(GenericDeviceModelStatusEnumType::Accepted);

        assert!(response.validate().is_ok());
    }

    #[test]
    fn test_get_report_response_all_status_types() {
        let statuses = vec![
            GenericDeviceModelStatusEnumType::Accepted,
            GenericDeviceModelStatusEnumType::Rejected,
            GenericDeviceModelStatusEnumType::NotSupported,
            GenericDeviceModelStatusEnumType::EmptyResultSet,
        ];

        for status in statuses {
            let response = GetReportResponse::new(status.clone());
            assert_eq!(response.status, status);
            assert!(response.validate().is_ok());
        }
    }

    #[test]
    fn test_get_report_response_json_round_trip() {
        let status_info = create_test_status_info();
        let custom_data = create_test_custom_data();
        let response = GetReportResponse::new(GenericDeviceModelStatusEnumType::NotSupported)
            .with_status_info(status_info)
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&response).unwrap();
        let parsed: GetReportResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(response, parsed);
        assert!(parsed.validate().is_ok());
    }
}