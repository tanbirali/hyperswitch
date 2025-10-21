<<<<<<< HEAD
use std::str::FromStr;

use hyperswitch_domain_models::{
    address::{Address, AddressDetails, PhoneDetails},
    payment_method_data::{Card, PaymentMethodData},
    router_request_types::AuthenticationData,
};
use masking::Secret;
use router::types::{self, storage::enums, PaymentAddress};

use crate::{
    connector_auth,
    utils::{self, ConnectorActions, PaymentInfo},
};

#[derive(Clone, Copy)]
struct AciTest;
impl ConnectorActions for AciTest {}
impl utils::Connector for AciTest {
    fn get_data(&self) -> types::api::ConnectorData {
        use router::connector::Aci;
        utils::construct_connector_data_old(
            Box::new(Aci::new()),
            types::Connector::Aci,
            types::api::GetToken::Connector,
            None,
        )
    }

    fn get_auth_token(&self) -> types::ConnectorAuthType {
        utils::to_connector_auth_type(
            connector_auth::ConnectorAuthentication::new()
                .aci
                .expect("Missing connector authentication configuration")
                .into(),
        )
    }

    fn get_name(&self) -> String {
        "aci".to_string()
    }
}

static CONNECTOR: AciTest = AciTest {};

fn get_default_payment_info() -> Option<PaymentInfo> {
    Some(PaymentInfo {
        address: Some(PaymentAddress::new(
=======
#![allow(clippy::print_stdout)]

use std::{borrow::Cow, marker::PhantomData, str::FromStr, sync::Arc};

use common_utils::id_type;
use hyperswitch_domain_models::address::{Address, AddressDetails, PhoneDetails};
use masking::Secret;
use router::{
    configs::settings::Settings,
    core::payments,
    db::StorageImpl,
    routes, services,
    types::{self, storage::enums, PaymentAddress},
};
use tokio::sync::oneshot;

use crate::{connector_auth::ConnectorAuthentication, utils};

fn construct_payment_router_data() -> types::PaymentsAuthorizeRouterData {
    let auth = ConnectorAuthentication::new()
        .aci
        .expect("Missing ACI connector authentication configuration");

    let merchant_id = id_type::MerchantId::try_from(Cow::from("aci")).unwrap();

    types::RouterData {
        flow: PhantomData,
        merchant_id,
        customer_id: Some(id_type::CustomerId::try_from(Cow::from("aci")).unwrap()),
        tenant_id: id_type::TenantId::try_from_string("public".to_string()).unwrap(),
        connector: "aci".to_string(),
        payment_id: uuid::Uuid::new_v4().to_string(),
        attempt_id: uuid::Uuid::new_v4().to_string(),
        status: enums::AttemptStatus::default(),
        auth_type: enums::AuthenticationType::NoThreeDs,
        payment_method: enums::PaymentMethod::Card,
        connector_auth_type: utils::to_connector_auth_type(auth.into()),
        description: Some("This is a test".to_string()),
        payment_method_status: None,
        request: types::PaymentsAuthorizeData {
            amount: 1000,
            currency: enums::Currency::USD,
            payment_method_data: types::domain::PaymentMethodData::Card(types::domain::Card {
                card_number: cards::CardNumber::from_str("4200000000000000").unwrap(),
                card_exp_month: Secret::new("10".to_string()),
                card_exp_year: Secret::new("2025".to_string()),
                card_cvc: Secret::new("999".to_string()),
                card_issuer: None,
                card_network: None,
                card_type: None,
                card_issuing_country: None,
                bank_code: None,
                nick_name: Some(Secret::new("nick_name".into())),
                card_holder_name: Some(Secret::new("card holder name".into())),
                co_badged_card_data: None,
            }),
            confirm: true,
            statement_descriptor_suffix: None,
            statement_descriptor: None,
            setup_future_usage: None,
            mandate_id: None,
            off_session: None,
            setup_mandate_details: None,
            capture_method: None,
            browser_info: None,
            order_details: None,
            order_category: None,
            email: None,
            customer_name: None,
            session_token: None,
            enrolled_for_3ds: false,
            related_transaction_id: None,
            payment_experience: None,
            payment_method_type: None,
            router_return_url: None,
            webhook_url: None,
            complete_authorize_url: None,
            customer_id: None,
            surcharge_details: None,
            request_incremental_authorization: false,
            metadata: None,
            authentication_data: None,
            customer_acceptance: None,
            locale: None,
            enable_partial_authorization: None,
            ..utils::PaymentAuthorizeType::default().0
        },
        response: Err(types::ErrorResponse::default()),
        address: PaymentAddress::new(
            None,
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
            None,
            Some(Address {
                address: Some(AddressDetails {
                    first_name: Some(Secret::new("John".to_string())),
                    last_name: Some(Secret::new("Doe".to_string())),
<<<<<<< HEAD
                    line1: Some(Secret::new("123 Main St".to_string())),
                    city: Some("New York".to_string()),
                    state: Some(Secret::new("NY".to_string())),
                    zip: Some(Secret::new("10001".to_string())),
                    country: Some(enums::CountryAlpha2::US),
                    ..Default::default()
                }),
                phone: Some(PhoneDetails {
                    number: Some(Secret::new("+1234567890".to_string())),
                    country_code: Some("+1".to_string()),
=======
                    ..Default::default()
                }),
                phone: Some(PhoneDetails {
                    number: Some(Secret::new("9123456789".to_string())),
                    country_code: Some("+351".to_string()),
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
                }),
                email: None,
            }),
            None,
<<<<<<< HEAD
            None,
        )),
        ..Default::default()
    })
}

fn get_payment_authorize_data() -> Option<types::PaymentsAuthorizeData> {
    Some(types::PaymentsAuthorizeData {
        payment_method_data: PaymentMethodData::Card(Card {
            card_number: cards::CardNumber::from_str("4200000000000000").unwrap(),
            card_exp_month: Secret::new("10".to_string()),
            card_exp_year: Secret::new("2025".to_string()),
            card_cvc: Secret::new("999".to_string()),
            card_holder_name: Some(Secret::new("John Doe".to_string())),
            ..utils::CCardType::default().0
        }),
        ..utils::PaymentAuthorizeType::default().0
    })
}

fn get_threeds_payment_authorize_data() -> Option<types::PaymentsAuthorizeData> {
    Some(types::PaymentsAuthorizeData {
        payment_method_data: PaymentMethodData::Card(Card {
            card_number: cards::CardNumber::from_str("4200000000000000").unwrap(),
            card_exp_month: Secret::new("10".to_string()),
            card_exp_year: Secret::new("2025".to_string()),
            card_cvc: Secret::new("999".to_string()),
            card_holder_name: Some(Secret::new("John Doe".to_string())),
            ..utils::CCardType::default().0
        }),
        enrolled_for_3ds: true,
        authentication_data: Some(AuthenticationData {
            eci: Some("05".to_string()),
            cavv: Secret::new("jJ81HADVRtXfCBATEp01CJUAAAA".to_string()),
            threeds_server_transaction_id: Some("9458d8d4-f19f-4c28-b5c7-421b1dd2e1aa".to_string()),
            message_version: Some(common_utils::types::SemanticVersion::new(2, 1, 0)),
            ds_trans_id: Some("97267598FAE648F28083B2D2AF7B1234".to_string()),
            created_at: common_utils::date_time::now(),
            challenge_code: Some("01".to_string()),
            challenge_cancel: None,
            challenge_code_reason: Some("01".to_string()),
            message_extension: None,
            acs_trans_id: None,
            authentication_type: None,
        }),
        ..utils::PaymentAuthorizeType::default().0
    })
}

#[actix_web::test]
async fn should_only_authorize_payment() {
    let response = CONNECTOR
        .authorize_payment(get_payment_authorize_data(), get_default_payment_info())
        .await
        .expect("Authorize payment response");
    assert_eq!(response.status, enums::AttemptStatus::Authorized);
}

#[actix_web::test]
async fn should_capture_authorized_payment() {
    let response = CONNECTOR
        .authorize_and_capture_payment(
            get_payment_authorize_data(),
            None,
            get_default_payment_info(),
        )
        .await
        .expect("Capture payment response");
    assert_eq!(response.status, enums::AttemptStatus::Charged);
}

#[actix_web::test]
async fn should_partially_capture_authorized_payment() {
    let response = CONNECTOR
        .authorize_and_capture_payment(
            get_payment_authorize_data(),
            Some(types::PaymentsCaptureData {
                amount_to_capture: 50,
                ..utils::PaymentCaptureType::default().0
            }),
            get_default_payment_info(),
        )
        .await
        .expect("Capture payment response");
    assert_eq!(response.status, enums::AttemptStatus::Charged);
}

#[actix_web::test]
async fn should_sync_authorized_payment() {
    let authorize_response = CONNECTOR
        .authorize_payment(get_payment_authorize_data(), get_default_payment_info())
        .await
        .expect("Authorize payment response");
    let txn_id = utils::get_connector_transaction_id(authorize_response.response);
    let response = CONNECTOR
        .psync_retry_till_status_matches(
            enums::AttemptStatus::Authorized,
            Some(types::PaymentsSyncData {
                connector_transaction_id: types::ResponseId::ConnectorTransactionId(
                    txn_id.unwrap(),
                ),
                ..Default::default()
            }),
            get_default_payment_info(),
        )
        .await
        .expect("PSync response");
    assert_eq!(response.status, enums::AttemptStatus::Authorized,);
}

#[actix_web::test]
async fn should_void_authorized_payment() {
    let response = CONNECTOR
        .authorize_and_void_payment(
            get_payment_authorize_data(),
            Some(types::PaymentsCancelData {
                connector_transaction_id: String::from(""),
                cancellation_reason: Some("requested_by_customer".to_string()),
                ..Default::default()
            }),
            get_default_payment_info(),
        )
        .await
        .expect("Void payment response");
    assert_eq!(response.status, enums::AttemptStatus::Voided);
}

#[actix_web::test]
async fn should_refund_manually_captured_payment() {
    let response = CONNECTOR
        .capture_payment_and_refund(
            get_payment_authorize_data(),
            None,
            None,
            get_default_payment_info(),
        )
        .await
        .unwrap();
    assert_eq!(
        response.response.unwrap().refund_status,
        enums::RefundStatus::Success,
    );
}

#[actix_web::test]
async fn should_partially_refund_manually_captured_payment() {
    let response = CONNECTOR
        .capture_payment_and_refund(
            get_payment_authorize_data(),
            None,
            Some(types::RefundsData {
                refund_amount: 50,
                ..utils::PaymentRefundType::default().0
            }),
            get_default_payment_info(),
        )
        .await
        .unwrap();
    assert_eq!(
        response.response.unwrap().refund_status,
        enums::RefundStatus::Success,
    );
}

#[actix_web::test]
async fn should_sync_manually_captured_refund() {
    let refund_response = CONNECTOR
        .capture_payment_and_refund(
            get_payment_authorize_data(),
            None,
            None,
            get_default_payment_info(),
        )
        .await
        .unwrap();
    let response = CONNECTOR
        .rsync_retry_till_status_matches(
            enums::RefundStatus::Success,
            refund_response.response.unwrap().connector_refund_id,
            None,
            get_default_payment_info(),
        )
        .await
        .unwrap();
    assert_eq!(
        response.response.unwrap().refund_status,
        enums::RefundStatus::Success,
    );
}

#[actix_web::test]
async fn should_make_payment() {
    let authorize_response = CONNECTOR
        .make_payment(get_payment_authorize_data(), get_default_payment_info())
        .await
        .unwrap();
    assert_eq!(authorize_response.status, enums::AttemptStatus::Charged);
}

#[actix_web::test]
async fn should_sync_auto_captured_payment() {
    let authorize_response = CONNECTOR
        .make_payment(get_payment_authorize_data(), get_default_payment_info())
        .await
        .unwrap();
    assert_eq!(authorize_response.status, enums::AttemptStatus::Charged);
    let txn_id = utils::get_connector_transaction_id(authorize_response.response);
    assert_ne!(txn_id, None, "Empty connector transaction id");
    let response = CONNECTOR
        .psync_retry_till_status_matches(
            enums::AttemptStatus::Charged,
            Some(types::PaymentsSyncData {
                connector_transaction_id: types::ResponseId::ConnectorTransactionId(
                    txn_id.unwrap(),
                ),
                capture_method: Some(enums::CaptureMethod::Automatic),
                ..Default::default()
            }),
            get_default_payment_info(),
        )
        .await
        .unwrap();
    assert_eq!(response.status, enums::AttemptStatus::Charged,);
}

#[actix_web::test]
async fn should_refund_auto_captured_payment() {
    let response = CONNECTOR
        .make_payment_and_refund(
            get_payment_authorize_data(),
            None,
            get_default_payment_info(),
        )
        .await
        .unwrap();
    assert_eq!(
        response.response.unwrap().refund_status,
        enums::RefundStatus::Success,
    );
}

#[actix_web::test]
async fn should_partially_refund_succeeded_payment() {
    let refund_response = CONNECTOR
        .make_payment_and_refund(
            get_payment_authorize_data(),
            Some(types::RefundsData {
                refund_amount: 50,
                ..utils::PaymentRefundType::default().0
            }),
            get_default_payment_info(),
        )
        .await
        .unwrap();
    assert_eq!(
        refund_response.response.unwrap().refund_status,
        enums::RefundStatus::Success,
    );
}

#[actix_web::test]
async fn should_refund_succeeded_payment_multiple_times() {
    CONNECTOR
        .make_payment_and_multiple_refund(
            get_payment_authorize_data(),
            Some(types::RefundsData {
                refund_amount: 50,
                ..utils::PaymentRefundType::default().0
            }),
            get_default_payment_info(),
        )
        .await;
}

#[actix_web::test]
async fn should_sync_refund() {
    let refund_response = CONNECTOR
        .make_payment_and_refund(
            get_payment_authorize_data(),
            None,
            get_default_payment_info(),
        )
        .await
        .unwrap();
    let response = CONNECTOR
        .rsync_retry_till_status_matches(
            enums::RefundStatus::Success,
            refund_response.response.unwrap().connector_refund_id,
            None,
            get_default_payment_info(),
        )
        .await
        .unwrap();
    assert_eq!(
        response.response.unwrap().refund_status,
        enums::RefundStatus::Success,
    );
}

#[actix_web::test]
async fn should_fail_payment_for_incorrect_cvc() {
    let response = CONNECTOR
        .make_payment(
            Some(types::PaymentsAuthorizeData {
                payment_method_data: PaymentMethodData::Card(Card {
                    card_cvc: Secret::new("12345".to_string()),
                    ..utils::CCardType::default().0
                }),
                ..utils::PaymentAuthorizeType::default().0
            }),
            get_default_payment_info(),
        )
        .await
        .unwrap();
    assert!(
        response.response.is_err(),
        "Payment should fail with incorrect CVC"
    );
}

#[actix_web::test]
async fn should_fail_payment_for_invalid_exp_month() {
    let response = CONNECTOR
        .make_payment(
            Some(types::PaymentsAuthorizeData {
                payment_method_data: PaymentMethodData::Card(Card {
                    card_exp_month: Secret::new("20".to_string()),
                    ..utils::CCardType::default().0
                }),
                ..utils::PaymentAuthorizeType::default().0
            }),
            get_default_payment_info(),
        )
        .await
        .unwrap();
    assert!(
        response.response.is_err(),
        "Payment should fail with invalid expiry month"
    );
}

#[actix_web::test]
async fn should_fail_payment_for_incorrect_expiry_year() {
    let response = CONNECTOR
        .make_payment(
            Some(types::PaymentsAuthorizeData {
                payment_method_data: PaymentMethodData::Card(Card {
                    card_exp_year: Secret::new("2000".to_string()),
                    ..utils::CCardType::default().0
                }),
                ..utils::PaymentAuthorizeType::default().0
            }),
            get_default_payment_info(),
        )
        .await
        .unwrap();
    assert!(
        response.response.is_err(),
        "Payment should fail with incorrect expiry year"
    );
}

#[actix_web::test]
async fn should_fail_void_payment_for_auto_capture() {
    let authorize_response = CONNECTOR
        .make_payment(get_payment_authorize_data(), get_default_payment_info())
        .await
        .unwrap();
    assert_eq!(authorize_response.status, enums::AttemptStatus::Charged);
    let txn_id = utils::get_connector_transaction_id(authorize_response.response);
    assert_ne!(txn_id, None, "Empty connector transaction id");
    let void_response = CONNECTOR
        .void_payment(txn_id.unwrap(), None, get_default_payment_info())
        .await
        .unwrap();
    assert!(
        void_response.response.is_err(),
        "Void should fail for already captured payment"
    );
}

#[actix_web::test]
async fn should_fail_capture_for_invalid_payment() {
    let capture_response = CONNECTOR
        .capture_payment("123456789".to_string(), None, get_default_payment_info())
        .await
        .unwrap();
    assert!(
        capture_response.response.is_err(),
        "Capture should fail for invalid payment ID"
    );
}

#[actix_web::test]
async fn should_fail_for_refund_amount_higher_than_payment_amount() {
    let response = CONNECTOR
        .make_payment_and_refund(
            get_payment_authorize_data(),
            Some(types::RefundsData {
                refund_amount: 150,
                ..utils::PaymentRefundType::default().0
            }),
            get_default_payment_info(),
        )
        .await
        .unwrap();
    assert!(
        response.response.is_err(),
        "Refund should fail when amount exceeds payment amount"
    );
}

#[actix_web::test]
#[ignore]
async fn should_make_threeds_payment() {
    let authorize_response = CONNECTOR
        .make_payment(
            get_threeds_payment_authorize_data(),
            get_default_payment_info(),
        )
        .await
        .unwrap();

    assert!(
        authorize_response.status == enums::AttemptStatus::AuthenticationPending
            || authorize_response.status == enums::AttemptStatus::Charged,
        "3DS payment should result in AuthenticationPending or Charged status, got: {:?}",
        authorize_response.status
    );

    if let Ok(types::PaymentsResponseData::TransactionResponse {
        redirection_data, ..
    }) = &authorize_response.response
    {
        if authorize_response.status == enums::AttemptStatus::AuthenticationPending {
            assert!(
                redirection_data.is_some(),
                "3DS flow should include redirection data for authentication"
            );
        }
=======
        ),
        connector_meta_data: None,
        connector_wallets_details: None,
        amount_captured: None,
        minor_amount_captured: None,
        access_token: None,
        session_token: None,
        reference_id: None,
        payment_method_token: None,
        connector_customer: None,
        recurring_mandate_payment_data: None,
        connector_response: None,
        preprocessing_id: None,
        connector_request_reference_id: uuid::Uuid::new_v4().to_string(),
        #[cfg(feature = "payouts")]
        payout_method_data: None,
        #[cfg(feature = "payouts")]
        quote_id: None,
        test_mode: None,
        payment_method_balance: None,
        connector_api_version: None,
        connector_http_status_code: None,
        apple_pay_flow: None,
        external_latency: None,
        frm_metadata: None,
        refund_id: None,
        dispute_id: None,
        integrity_check: Ok(()),
        additional_merchant_data: None,
        header_payload: None,
        connector_mandate_request_reference_id: None,
        authentication_id: None,
        psd2_sca_exemption_type: None,
        raw_connector_response: None,
        is_payment_id_from_merchant: None,
        l2_l3_data: None,
        minor_amount_capturable: None,
    }
}

fn construct_refund_router_data<F>() -> types::RefundsRouterData<F> {
    let auth = ConnectorAuthentication::new()
        .aci
        .expect("Missing ACI connector authentication configuration");

    let merchant_id = id_type::MerchantId::try_from(Cow::from("aci")).unwrap();

    types::RouterData {
        flow: PhantomData,
        merchant_id,
        customer_id: Some(id_type::CustomerId::try_from(Cow::from("aci")).unwrap()),
        tenant_id: id_type::TenantId::try_from_string("public".to_string()).unwrap(),
        connector: "aci".to_string(),
        payment_id: uuid::Uuid::new_v4().to_string(),
        attempt_id: uuid::Uuid::new_v4().to_string(),
        payment_method_status: None,
        status: enums::AttemptStatus::default(),
        payment_method: enums::PaymentMethod::Card,
        auth_type: enums::AuthenticationType::NoThreeDs,
        connector_auth_type: utils::to_connector_auth_type(auth.into()),
        description: Some("This is a test".to_string()),
        request: types::RefundsData {
            payment_amount: 1000,
            currency: enums::Currency::USD,

            refund_id: uuid::Uuid::new_v4().to_string(),
            connector_transaction_id: String::new(),
            refund_amount: 100,
            webhook_url: None,
            connector_metadata: None,
            reason: None,
            connector_refund_id: None,
            browser_info: None,
            ..utils::PaymentRefundType::default().0
        },
        response: Err(types::ErrorResponse::default()),
        address: PaymentAddress::default(),
        connector_meta_data: None,
        connector_wallets_details: None,
        amount_captured: None,
        minor_amount_captured: None,
        access_token: None,
        session_token: None,
        reference_id: None,
        payment_method_token: None,
        connector_customer: None,
        recurring_mandate_payment_data: None,
        connector_response: None,
        preprocessing_id: None,
        connector_request_reference_id: uuid::Uuid::new_v4().to_string(),
        #[cfg(feature = "payouts")]
        payout_method_data: None,
        #[cfg(feature = "payouts")]
        quote_id: None,
        test_mode: None,
        payment_method_balance: None,
        connector_api_version: None,
        connector_http_status_code: None,
        apple_pay_flow: None,
        external_latency: None,
        frm_metadata: None,
        refund_id: None,
        dispute_id: None,
        integrity_check: Ok(()),
        additional_merchant_data: None,
        header_payload: None,
        connector_mandate_request_reference_id: None,
        authentication_id: None,
        psd2_sca_exemption_type: None,
        raw_connector_response: None,
        is_payment_id_from_merchant: None,
        l2_l3_data: None,
        minor_amount_capturable: None,
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
    }
}

#[actix_web::test]
<<<<<<< HEAD
#[ignore]
async fn should_authorize_threeds_payment() {
    let response = CONNECTOR
        .authorize_payment(
            get_threeds_payment_authorize_data(),
            get_default_payment_info(),
        )
        .await
        .expect("Authorize 3DS payment response");

    assert!(
        response.status == enums::AttemptStatus::AuthenticationPending
            || response.status == enums::AttemptStatus::Authorized,
        "3DS authorization should result in AuthenticationPending or Authorized status, got: {:?}",
        response.status
=======
async fn payments_create_success() {
    let conf = Settings::new().unwrap();
    let tx: oneshot::Sender<()> = oneshot::channel().0;

    let app_state = Box::pin(routes::AppState::with_storage(
        conf,
        StorageImpl::PostgresqlTest,
        tx,
        Box::new(services::MockApiClient),
    ))
    .await;
    let state = Arc::new(app_state)
        .get_session_state(
            &id_type::TenantId::try_from_string("public".to_string()).unwrap(),
            None,
            || {},
        )
        .unwrap();

    use router::connector::Aci;
    let connector = utils::construct_connector_data_old(
        Box::new(Aci::new()),
        types::Connector::Aci,
        types::api::GetToken::Connector,
        None,
    );
    let connector_integration: services::BoxedPaymentConnectorIntegrationInterface<
        types::api::Authorize,
        types::PaymentsAuthorizeData,
        types::PaymentsResponseData,
    > = connector.connector.get_connector_integration();
    let request = construct_payment_router_data();
    let response = services::api::execute_connector_processing_step(
        &state,
        connector_integration,
        &request,
        payments::CallConnectorAction::Trigger,
        None,
        None,
    )
    .await
    .unwrap();
    assert!(
        response.status == enums::AttemptStatus::Charged,
        "The payment failed"
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
    );
}

#[actix_web::test]
#[ignore]
<<<<<<< HEAD
async fn should_sync_threeds_payment() {
    let authorize_response = CONNECTOR
        .authorize_payment(
            get_threeds_payment_authorize_data(),
            get_default_payment_info(),
        )
        .await
        .expect("Authorize 3DS payment response");
    let txn_id = utils::get_connector_transaction_id(authorize_response.response);
    let response = CONNECTOR
        .psync_retry_till_status_matches(
            enums::AttemptStatus::AuthenticationPending,
            Some(types::PaymentsSyncData {
                connector_transaction_id: types::ResponseId::ConnectorTransactionId(
                    txn_id.unwrap(),
                ),
                ..Default::default()
            }),
            get_default_payment_info(),
        )
        .await
        .expect("PSync 3DS response");
    assert!(
        response.status == enums::AttemptStatus::AuthenticationPending
            || response.status == enums::AttemptStatus::Authorized,
        "3DS sync should maintain AuthenticationPending or Authorized status"
    );
}
=======
async fn payments_create_failure() {
    {
        let conf = Settings::new().unwrap();
        use router::connector::Aci;
        let tx: oneshot::Sender<()> = oneshot::channel().0;

        let app_state = Box::pin(routes::AppState::with_storage(
            conf,
            StorageImpl::PostgresqlTest,
            tx,
            Box::new(services::MockApiClient),
        ))
        .await;
        let state = Arc::new(app_state)
            .get_session_state(
                &id_type::TenantId::try_from_string("public".to_string()).unwrap(),
                None,
                || {},
            )
            .unwrap();
        let connector = utils::construct_connector_data_old(
            Box::new(Aci::new()),
            types::Connector::Aci,
            types::api::GetToken::Connector,
            None,
        );
        let connector_integration: services::BoxedPaymentConnectorIntegrationInterface<
            types::api::Authorize,
            types::PaymentsAuthorizeData,
            types::PaymentsResponseData,
        > = connector.connector.get_connector_integration();
        let mut request = construct_payment_router_data();
        request.request.payment_method_data =
            types::domain::PaymentMethodData::Card(types::domain::Card {
                card_number: cards::CardNumber::from_str("4200000000000000").unwrap(),
                card_exp_month: Secret::new("10".to_string()),
                card_exp_year: Secret::new("2025".to_string()),
                card_cvc: Secret::new("99".to_string()),
                card_issuer: None,
                card_network: None,
                card_type: None,
                card_issuing_country: None,
                bank_code: None,
                nick_name: Some(Secret::new("nick_name".into())),
                card_holder_name: Some(Secret::new("card holder name".into())),
                co_badged_card_data: None,
            });

        let response = services::api::execute_connector_processing_step(
            &state,
            connector_integration,
            &request,
            payments::CallConnectorAction::Trigger,
            None,
            None,
        )
        .await
        .is_err();
        println!("{response:?}");
        assert!(response, "The payment was intended to fail but it passed");
    }
}

#[actix_web::test]
async fn refund_for_successful_payments() {
    let conf = Settings::new().unwrap();
    use router::connector::Aci;
    let connector = utils::construct_connector_data_old(
        Box::new(Aci::new()),
        types::Connector::Aci,
        types::api::GetToken::Connector,
        None,
    );
    let tx: oneshot::Sender<()> = oneshot::channel().0;

    let app_state = Box::pin(routes::AppState::with_storage(
        conf,
        StorageImpl::PostgresqlTest,
        tx,
        Box::new(services::MockApiClient),
    ))
    .await;
    let state = Arc::new(app_state)
        .get_session_state(
            &id_type::TenantId::try_from_string("public".to_string()).unwrap(),
            None,
            || {},
        )
        .unwrap();
    let connector_integration: services::BoxedPaymentConnectorIntegrationInterface<
        types::api::Authorize,
        types::PaymentsAuthorizeData,
        types::PaymentsResponseData,
    > = connector.connector.get_connector_integration();
    let request = construct_payment_router_data();
    let response = services::api::execute_connector_processing_step(
        &state,
        connector_integration,
        &request,
        payments::CallConnectorAction::Trigger,
        None,
        None,
    )
    .await
    .unwrap();
    assert!(
        response.status == enums::AttemptStatus::Charged,
        "The payment failed"
    );
    let connector_integration: services::BoxedRefundConnectorIntegrationInterface<
        types::api::Execute,
        types::RefundsData,
        types::RefundsResponseData,
    > = connector.connector.get_connector_integration();
    let mut refund_request = construct_refund_router_data();
    refund_request.request.connector_transaction_id = match response.response.unwrap() {
        types::PaymentsResponseData::TransactionResponse { resource_id, .. } => {
            resource_id.get_connector_transaction_id().unwrap()
        }
        _ => panic!("Connector transaction id not found"),
    };
    let response = services::api::execute_connector_processing_step(
        &state,
        connector_integration,
        &refund_request,
        payments::CallConnectorAction::Trigger,
        None,
        None,
    )
    .await
    .unwrap();
    println!("{response:?}");
    assert!(
        response.response.unwrap().refund_status == enums::RefundStatus::Success,
        "The refund transaction failed"
    );
}

#[actix_web::test]
#[ignore]
async fn refunds_create_failure() {
    let conf = Settings::new().unwrap();
    use router::connector::Aci;
    let connector = utils::construct_connector_data_old(
        Box::new(Aci::new()),
        types::Connector::Aci,
        types::api::GetToken::Connector,
        None,
    );
    let tx: oneshot::Sender<()> = oneshot::channel().0;

    let app_state = Box::pin(routes::AppState::with_storage(
        conf,
        StorageImpl::PostgresqlTest,
        tx,
        Box::new(services::MockApiClient),
    ))
    .await;
    let state = Arc::new(app_state)
        .get_session_state(
            &id_type::TenantId::try_from_string("public".to_string()).unwrap(),
            None,
            || {},
        )
        .unwrap();
    let connector_integration: services::BoxedRefundConnectorIntegrationInterface<
        types::api::Execute,
        types::RefundsData,
        types::RefundsResponseData,
    > = connector.connector.get_connector_integration();
    let mut request = construct_refund_router_data();
    request.request.connector_transaction_id = "1234".to_string();
    let response = services::api::execute_connector_processing_step(
        &state,
        connector_integration,
        &request,
        payments::CallConnectorAction::Trigger,
        None,
        None,
    )
    .await
    .is_err();
    println!("{response:?}");
    assert!(response, "The refund was intended to fail but it passed");
}
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
