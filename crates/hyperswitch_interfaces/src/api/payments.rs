//! Payments interface

use hyperswitch_domain_models::{
    router_flow_types::{
        payments::{
            Approve, Authorize, AuthorizeSessionToken, CalculateTax, Capture, CompleteAuthorize,
<<<<<<< HEAD
            CreateConnectorCustomer, ExtendAuthorization, IncrementalAuthorization, PSync,
            PaymentMethodToken, PostCaptureVoid, PostProcessing, PostSessionTokens, PreProcessing,
            Reject, SdkSessionUpdate, Session, SetupMandate, UpdateMetadata, Void,
        },
        Authenticate, CreateOrder, ExternalVaultProxy, GiftCardBalanceCheck, PostAuthenticate,
        PreAuthenticate,
    },
    router_request_types::{
        AuthorizeSessionTokenData, CompleteAuthorizeData, ConnectorCustomerData,
        CreateOrderRequestData, ExternalVaultProxyPaymentsData, GiftCardBalanceCheckRequestData,
        PaymentMethodTokenizationData, PaymentsApproveData, PaymentsAuthenticateData,
        PaymentsAuthorizeData, PaymentsCancelData, PaymentsCancelPostCaptureData,
        PaymentsCaptureData, PaymentsExtendAuthorizationData, PaymentsIncrementalAuthorizationData,
        PaymentsPostAuthenticateData, PaymentsPostProcessingData, PaymentsPostSessionTokensData,
        PaymentsPreAuthenticateData, PaymentsPreProcessingData, PaymentsRejectData,
        PaymentsSessionData, PaymentsSyncData, PaymentsTaxCalculationData,
        PaymentsUpdateMetadataData, SdkPaymentsSessionUpdateData, SetupMandateRequestData,
    },
    router_response_types::{
        GiftCardBalanceCheckResponseData, PaymentsResponseData, TaxCalculationResponseData,
    },
=======
            CreateConnectorCustomer, IncrementalAuthorization, PSync, PaymentMethodToken,
            PostCaptureVoid, PostProcessing, PostSessionTokens, PreProcessing, Reject,
            SdkSessionUpdate, Session, SetupMandate, UpdateMetadata, Void,
        },
        CreateOrder, ExternalVaultProxy,
    },
    router_request_types::{
        AuthorizeSessionTokenData, CompleteAuthorizeData, ConnectorCustomerData,
        CreateOrderRequestData, ExternalVaultProxyPaymentsData, PaymentMethodTokenizationData,
        PaymentsApproveData, PaymentsAuthorizeData, PaymentsCancelData,
        PaymentsCancelPostCaptureData, PaymentsCaptureData, PaymentsIncrementalAuthorizationData,
        PaymentsPostProcessingData, PaymentsPostSessionTokensData, PaymentsPreProcessingData,
        PaymentsRejectData, PaymentsSessionData, PaymentsSyncData, PaymentsTaxCalculationData,
        PaymentsUpdateMetadataData, SdkPaymentsSessionUpdateData, SetupMandateRequestData,
    },
    router_response_types::{PaymentsResponseData, TaxCalculationResponseData},
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
};

use crate::api;

/// trait Payment
pub trait Payment:
    api::ConnectorCommon
    + api::ConnectorSpecifications
    + api::ConnectorValidation
    + PaymentAuthorize
    + PaymentAuthorizeSessionToken
    + PaymentsCompleteAuthorize
    + PaymentSync
    + PaymentCapture
    + PaymentVoid
    + PaymentPostCaptureVoid
    + PaymentApprove
    + PaymentReject
    + MandateSetup
    + PaymentSession
    + PaymentToken
    + PaymentsPreProcessing
    + PaymentsPostProcessing
    + ConnectorCustomer
    + PaymentIncrementalAuthorization
<<<<<<< HEAD
    + PaymentExtendAuthorization
=======
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
    + PaymentSessionUpdate
    + PaymentPostSessionTokens
    + PaymentUpdateMetadata
    + PaymentsCreateOrder
    + ExternalVaultProxyPaymentsCreateV1
<<<<<<< HEAD
    + PaymentsGiftCardBalanceCheck
=======
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
{
}

/// trait PaymentSession
pub trait PaymentSession:
    api::ConnectorIntegration<Session, PaymentsSessionData, PaymentsResponseData>
{
}

/// trait MandateSetup
pub trait MandateSetup:
    api::ConnectorIntegration<SetupMandate, SetupMandateRequestData, PaymentsResponseData>
{
}

/// trait PaymentAuthorize
pub trait PaymentAuthorize:
    api::ConnectorIntegration<Authorize, PaymentsAuthorizeData, PaymentsResponseData>
{
}

/// trait PaymentCapture
pub trait PaymentCapture:
    api::ConnectorIntegration<Capture, PaymentsCaptureData, PaymentsResponseData>
{
}

/// trait PaymentSync
pub trait PaymentSync:
    api::ConnectorIntegration<PSync, PaymentsSyncData, PaymentsResponseData>
{
}

/// trait PaymentVoid
pub trait PaymentVoid:
    api::ConnectorIntegration<Void, PaymentsCancelData, PaymentsResponseData>
{
}

/// trait PaymentPostCaptureVoid
pub trait PaymentPostCaptureVoid:
    api::ConnectorIntegration<PostCaptureVoid, PaymentsCancelPostCaptureData, PaymentsResponseData>
{
}

<<<<<<< HEAD
/// trait PaymentExtendAuthorization
pub trait PaymentExtendAuthorization:
    api::ConnectorIntegration<
    ExtendAuthorization,
    PaymentsExtendAuthorizationData,
    PaymentsResponseData,
>
{
}

=======
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
/// trait PaymentApprove
pub trait PaymentApprove:
    api::ConnectorIntegration<Approve, PaymentsApproveData, PaymentsResponseData>
{
}

/// trait PaymentReject
pub trait PaymentReject:
    api::ConnectorIntegration<Reject, PaymentsRejectData, PaymentsResponseData>
{
}

/// trait PaymentToken
pub trait PaymentToken:
    api::ConnectorIntegration<PaymentMethodToken, PaymentMethodTokenizationData, PaymentsResponseData>
{
}

/// trait PaymentAuthorizeSessionToken
pub trait PaymentAuthorizeSessionToken:
    api::ConnectorIntegration<AuthorizeSessionToken, AuthorizeSessionTokenData, PaymentsResponseData>
{
}

/// trait PaymentIncrementalAuthorization
pub trait PaymentIncrementalAuthorization:
    api::ConnectorIntegration<
    IncrementalAuthorization,
    PaymentsIncrementalAuthorizationData,
    PaymentsResponseData,
>
{
}

/// trait TaxCalculation
pub trait TaxCalculation:
    api::ConnectorIntegration<CalculateTax, PaymentsTaxCalculationData, TaxCalculationResponseData>
{
}

/// trait SessionUpdate
pub trait PaymentSessionUpdate:
    api::ConnectorIntegration<SdkSessionUpdate, SdkPaymentsSessionUpdateData, PaymentsResponseData>
{
}

/// trait PostSessionTokens
pub trait PaymentPostSessionTokens:
    api::ConnectorIntegration<PostSessionTokens, PaymentsPostSessionTokensData, PaymentsResponseData>
{
}

/// trait UpdateMetadata
pub trait PaymentUpdateMetadata:
    api::ConnectorIntegration<UpdateMetadata, PaymentsUpdateMetadataData, PaymentsResponseData>
{
}

/// trait PaymentsCompleteAuthorize
pub trait PaymentsCompleteAuthorize:
    api::ConnectorIntegration<CompleteAuthorize, CompleteAuthorizeData, PaymentsResponseData>
{
}

/// trait ConnectorCustomer
pub trait ConnectorCustomer:
    api::ConnectorIntegration<CreateConnectorCustomer, ConnectorCustomerData, PaymentsResponseData>
{
}

/// trait PaymentsPreProcessing
pub trait PaymentsPreProcessing:
    api::ConnectorIntegration<PreProcessing, PaymentsPreProcessingData, PaymentsResponseData>
{
}

<<<<<<< HEAD
/// trait PaymentsPreAuthenticate
pub trait PaymentsPreAuthenticate:
    api::ConnectorIntegration<PreAuthenticate, PaymentsPreAuthenticateData, PaymentsResponseData>
{
}

/// trait PaymentsAuthenticate
pub trait PaymentsAuthenticate:
    api::ConnectorIntegration<Authenticate, PaymentsAuthenticateData, PaymentsResponseData>
{
}

/// trait PaymentsPostAuthenticate
pub trait PaymentsPostAuthenticate:
    api::ConnectorIntegration<PostAuthenticate, PaymentsPostAuthenticateData, PaymentsResponseData>
{
}

=======
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
/// trait PaymentsPostProcessing
pub trait PaymentsPostProcessing:
    api::ConnectorIntegration<PostProcessing, PaymentsPostProcessingData, PaymentsResponseData>
{
}

/// trait PaymentsCreateOrder
pub trait PaymentsCreateOrder:
    api::ConnectorIntegration<CreateOrder, CreateOrderRequestData, PaymentsResponseData>
{
}

/// trait ExternalVaultProxyPaymentsCreate
pub trait ExternalVaultProxyPaymentsCreateV1:
    api::ConnectorIntegration<ExternalVaultProxy, ExternalVaultProxyPaymentsData, PaymentsResponseData>
{
}
<<<<<<< HEAD

/// trait PaymentsGiftCardBalanceCheck
pub trait PaymentsGiftCardBalanceCheck:
    api::ConnectorIntegration<
    GiftCardBalanceCheck,
    GiftCardBalanceCheckRequestData,
    GiftCardBalanceCheckResponseData,
>
{
}
=======
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
