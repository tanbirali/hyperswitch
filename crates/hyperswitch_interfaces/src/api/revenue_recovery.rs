//! Revenue Recovery Interface

use hyperswitch_domain_models::{
    router_flow_types::{
<<<<<<< HEAD
        BillingConnectorInvoiceSync, BillingConnectorPaymentsSync, InvoiceRecordBack,
    },
    router_request_types::revenue_recovery::{
        BillingConnectorInvoiceSyncRequest, BillingConnectorPaymentsSyncRequest,
        InvoiceRecordBackRequest,
    },
    router_response_types::revenue_recovery::{
        BillingConnectorInvoiceSyncResponse, BillingConnectorPaymentsSyncResponse,
        InvoiceRecordBackResponse,
=======
        BillingConnectorInvoiceSync, BillingConnectorPaymentsSync, RecoveryRecordBack,
    },
    router_request_types::revenue_recovery::{
        BillingConnectorInvoiceSyncRequest, BillingConnectorPaymentsSyncRequest,
        RevenueRecoveryRecordBackRequest,
    },
    router_response_types::revenue_recovery::{
        BillingConnectorInvoiceSyncResponse, BillingConnectorPaymentsSyncResponse,
        RevenueRecoveryRecordBackResponse,
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
    },
};

#[cfg(all(feature = "v2", feature = "revenue_recovery"))]
use super::ConnectorCommon;
use super::ConnectorIntegration;

/// trait RevenueRecovery
#[cfg(all(feature = "v2", feature = "revenue_recovery"))]
pub trait RevenueRecovery:
    ConnectorCommon
    + BillingConnectorPaymentsSyncIntegration
    + RevenueRecoveryRecordBack
    + BillingConnectorInvoiceSyncIntegration
{
}

/// trait BillingConnectorPaymentsSyncIntegration
pub trait BillingConnectorPaymentsSyncIntegration:
    ConnectorIntegration<
    BillingConnectorPaymentsSync,
    BillingConnectorPaymentsSyncRequest,
    BillingConnectorPaymentsSyncResponse,
>
{
}

/// trait RevenueRecoveryRecordBack
pub trait RevenueRecoveryRecordBack:
<<<<<<< HEAD
    ConnectorIntegration<InvoiceRecordBack, InvoiceRecordBackRequest, InvoiceRecordBackResponse>
=======
    ConnectorIntegration<
    RecoveryRecordBack,
    RevenueRecoveryRecordBackRequest,
    RevenueRecoveryRecordBackResponse,
>
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
{
}

/// trait BillingConnectorInvoiceSyncIntegration
pub trait BillingConnectorInvoiceSyncIntegration:
    ConnectorIntegration<
    BillingConnectorInvoiceSync,
    BillingConnectorInvoiceSyncRequest,
    BillingConnectorInvoiceSyncResponse,
>
{
}

#[cfg(not(all(feature = "v2", feature = "revenue_recovery")))]
/// trait RevenueRecovery
pub trait RevenueRecovery {}
