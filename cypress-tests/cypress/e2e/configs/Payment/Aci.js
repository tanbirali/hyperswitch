import { customerAcceptance } from "./Commons";

const successfulNo3DSCardDetails = {
  card_number: "4242424242424242",
<<<<<<< HEAD
  card_exp_month: "01",
  card_exp_year: "2045",
=======
  card_exp_month: "10",
  card_exp_year: "2050",
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
  card_holder_name: "morino",
  card_cvc: "737",
};

const successfulThreeDSTestCardDetails = {
<<<<<<< HEAD
  card_number: "5386024192625914",
  card_exp_month: "01",
  card_exp_year: "2045",
=======
  card_number: "4111111111111111",
  card_exp_month: "10",
  card_exp_year: "2050",
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
  card_holder_name: "morino",
  card_cvc: "737",
};

// This card details will fail because of card expiryYear
const failedNo3DSCardDetails = {
<<<<<<< HEAD
  card_number: "4012001037461114",
  card_exp_month: "01",
  card_exp_year: "35",
  card_holder_name: "joseph Doe",
  card_cvc: "737",
=======
  card_number: "4242424242424242",
  card_exp_month: "01",
  card_exp_year: "35",
  card_holder_name: "joseph Doe",
  card_cvc: "123",
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
};

const singleUseMandateData = {
  customer_acceptance: customerAcceptance,
  mandate_type: {
    single_use: {
      amount: 8000,
<<<<<<< HEAD
      currency: "EUR",
=======
      currency: "USD",
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
    },
  },
};

const multiUseMandateData = {
  customer_acceptance: customerAcceptance,
  mandate_type: {
    multi_use: {
      amount: 8000,
<<<<<<< HEAD
      currency: "EUR",
=======
      currency: "USD",
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
    },
  },
};

const billingAddress = {
  address: {
    line1: "1467",
    line2: "Harrison Street",
    line3: "Harrison Street",
    city: "San Fransico",
    state: "California",
    zip: "94122",
    country: "NL",
    first_name: "joseph",
    last_name: "Doe",
  },
  phone: {
    number: "9123456789",
    country_code: "+91",
  },
};

export const connectorDetails = {
  card_pm: {
    PaymentIntent: {
      Request: {
<<<<<<< HEAD
        currency: "EUR",
=======
        currency: "USD",
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
        customer_acceptance: null,
        setup_future_usage: "on_session",
      },
      Response: {
        status: 200,
        body: {
          status: "requires_payment_method",
          setup_future_usage: "on_session",
        },
      },
    },
<<<<<<< HEAD
    PaymentIntentOffSession: {
      Request: {
        amount: 6000,
        authentication_type: "no_three_ds",
        currency: "EUR",
        customer_acceptance: null,
        setup_future_usage: "off_session",
      },
      Response: {
        status: 200,
        body: {
          status: "requires_payment_method",
        },
      },
    },
=======
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
    No3DSAutoCapture: {
      Request: {
        payment_method: "card",
        payment_method_data: {
          card: successfulNo3DSCardDetails,
        },
<<<<<<< HEAD
        currency: "EUR",
=======
        currency: "USD",
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
        customer_acceptance: null,
        setup_future_usage: "on_session",
      },
      Response: {
        status: 200,
        body: {
          status: "succeeded",
          payment_method: "card",
          attempt_count: 1,
        },
      },
    },
<<<<<<< HEAD
    No3DSManualCapture: {
=======
    "3DSAutoCapture": {
      Configs: {
        TRIGGER_SKIP: true,
      },
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
      Request: {
        payment_method: "card",
        payment_method_data: {
          card: successfulNo3DSCardDetails,
        },
<<<<<<< HEAD
        currency: "EUR",
=======
        currency: "USD",
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
        customer_acceptance: null,
        setup_future_usage: "on_session",
      },
      Response: {
        status: 200,
        body: {
<<<<<<< HEAD
          status: "requires_capture",
=======
          status: "succeeded",
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
          payment_method: "card",
          attempt_count: 1,
        },
      },
    },
<<<<<<< HEAD
    "3DSAutoCapture": {
      Request: {
        payment_method: "card",
        payment_method_data: {
          card: successfulThreeDSTestCardDetails,
        },
        currency: "EUR",
        customer_acceptance: null,
        setup_future_usage: "on_session",
      },
      Response: {
        status: 200,
        body: {
          status: "requires_customer_action",
        },
      },
    },
    "3DSManualCapture": {
      Request: {
        payment_method: "card",
        payment_method_data: {
          card: successfulThreeDSTestCardDetails,
        },
        currency: "EUR",
        customer_acceptance: null,
        setup_future_usage: "on_session",
      },
      Response: {
        status: 200,
        body: {
          status: "requires_customer_action",
        },
      },
    },
    PaymentIntentWithShippingCost: {
      Request: {
        currency: "EUR",
=======
    PaymentIntentWithShippingCost: {
      Request: {
        currency: "USD",
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
        shipping_cost: 50,
        amount: 6000,
      },
      Response: {
        status: 200,
        body: {
          status: "requires_payment_method",
        },
      },
    },
    PaymentConfirmWithShippingCost: {
      Request: {
        payment_method: "card",
        payment_method_data: {
          card: successfulNo3DSCardDetails,
        },
<<<<<<< HEAD
        currency: "EUR",
=======
        currency: "USD",
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
        customer_acceptance: null,
        setup_future_usage: "on_session",
      },
      Response: {
        status: 200,
        body: {
          status: "succeeded",
          shipping_cost: 50,
          amount_received: 6050,
          amount: 6000,
        },
      },
    },
    Refund: {
      Request: {
        amount: 6000,
      },
      Response: {
        status: 200,
        body: {
          status: "succeeded",
        },
      },
    },
    PartialRefund: {
      Request: {
        amount: 2000,
      },
      Response: {
        status: 200,
        body: {
          status: "succeeded",
        },
      },
    },
    SyncRefund: {
      Response: {
        status: 200,
        body: {
          status: "succeeded",
        },
      },
    },
    Capture: {
      Request: {
        amount_to_capture: 6000,
      },
      Response: {
        status: 200,
        body: {
          status: "succeeded",
        },
      },
    },
<<<<<<< HEAD
    PartialCapture: {
      Request: {
        amount_to_capture: 2000,
      },
      Response: {
        status: 200,
        body: {
          status: "partially_captured",
        },
      },
    },
=======
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
    manualPaymentRefund: {
      Request: {
        amount: 6000,
      },
      Response: {
        status: 200,
        body: {
          status: "succeeded",
        },
      },
    },
    manualPaymentPartialRefund: {
      Request: {
        amount: 2000,
      },
      Response: {
        status: 200,
        body: {
          status: "succeeded",
        },
      },
    },
    MandateSingleUse3DSAutoCapture: {
      Configs: {
        TRIGGER_SKIP: true,
      },
      Request: {
        payment_method: "card",
        payment_method_data: {
          card: successfulThreeDSTestCardDetails,
        },
<<<<<<< HEAD
        currency: "EUR",
=======
        currency: "USD",
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
        mandate_data: singleUseMandateData,
      },
      Response: {
        status: 200,
        body: {
          status: "succeeded",
        },
      },
    },
<<<<<<< HEAD
    MandateSingleUse3DSManualCapture: {
      Configs: {
        TRIGGER_SKIP: true,
      },
      Request: {
        payment_method: "card",
        payment_method_data: {
          card: successfulThreeDSTestCardDetails,
        },
        currency: "EUR",
        mandate_data: singleUseMandateData,
      },
      Response: {
        status: 200,
        body: {
          status: "requires_capture",
        },
      },
    },
=======
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
    MandateSingleUseNo3DSAutoCapture: {
      Configs: {
        TRIGGER_SKIP: true,
      },
      Request: {
        payment_method: "card",
        payment_method_data: {
          card: successfulNo3DSCardDetails,
        },
<<<<<<< HEAD
        currency: "EUR",
        mandate_data: singleUseMandateData,
      },
      Response: {
        status: 200,
        body: {
          status: "succeeded",
        },
      },
    },
    MandateSingleUseNo3DSManualCapture: {
      Configs: {
        TRIGGER_SKIP: true,
      },
      Request: {
        payment_method: "card",
        payment_method_data: {
          card: successfulNo3DSCardDetails,
        },
=======
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
        currency: "USD",
        mandate_data: singleUseMandateData,
      },
      Response: {
        status: 200,
        body: {
<<<<<<< HEAD
          status: "requires_capture",
=======
          status: "succeeded",
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
        },
      },
    },
    MandateMultiUseNo3DSAutoCapture: {
      Configs: {
        TRIGGER_SKIP: true,
      },
      Request: {
        payment_method: "card",
        payment_method_data: {
          card: successfulNo3DSCardDetails,
        },
<<<<<<< HEAD
        currency: "EUR",
        mandate_data: multiUseMandateData,
      },
      Response: {
        status: 200,
        body: {
          status: "succeeded",
        },
      },
    },
    MandateMultiUseNo3DSManualCapture: {
      Configs: {
        TRIGGER_SKIP: true,
      },
      Request: {
        payment_method: "card",
        payment_method_data: {
          card: successfulNo3DSCardDetails,
        },
=======
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
        currency: "USD",
        mandate_data: multiUseMandateData,
      },
      Response: {
        status: 200,
        body: {
<<<<<<< HEAD
          status: "requires_capture",
=======
          status: "succeeded",
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
        },
      },
    },
    MandateMultiUse3DSAutoCapture: {
      Configs: {
        TRIGGER_SKIP: true,
      },
      Request: {
        payment_method: "card",
        payment_method_data: {
          card: successfulThreeDSTestCardDetails,
        },
<<<<<<< HEAD
        currency: "EUR",
=======
        currency: "USD",
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
        mandate_data: multiUseMandateData,
      },
      Response: {
        status: 200,
        body: {
          status: "succeeded",
        },
      },
    },
<<<<<<< HEAD
    MandateMultiUse3DSManualCapture: {
      Configs: {
        TRIGGER_SKIP: true,
      },
      Request: {
        payment_method: "card",
        payment_method_data: {
          card: successfulThreeDSTestCardDetails,
        },
        currency: "EUR",
        mandate_data: multiUseMandateData,
      },
      Response: {
        status: 200,
        body: {
          status: "requires_capture",
        },
      },
    },
    ZeroAuthMandate: {
      Request: {
        payment_method: "card",
        payment_method_data: {
          card: successfulNo3DSCardDetails,
        },
        currency: "EUR",
        mandate_data: singleUseMandateData,
      },
      Response: {
        status: 200,
        body: {
          status: "succeeded",
=======
    ZeroAuthMandate: {
      Response: {
        status: 501,
        body: {
          error: {
            type: "invalid_request",
            message: "Setup Mandate flow for Aci is not implemented",
            code: "IR_00",
          },
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
        },
      },
    },
    ZeroAuthPaymentIntent: {
      Request: {
        amount: 0,
        setup_future_usage: "off_session",
<<<<<<< HEAD
        currency: "EUR",
=======
        currency: "USD",
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
      },
      Response: {
        status: 200,
        body: {
          status: "requires_payment_method",
          setup_future_usage: "off_session",
        },
      },
    },
    ZeroAuthConfirmPayment: {
      Request: {
        payment_type: "setup_mandate",
        payment_method: "card",
        payment_method_type: "credit",
        payment_method_data: {
          card: successfulNo3DSCardDetails,
        },
      },
      Response: {
<<<<<<< HEAD
        status: 200,
        body: {
          status: "succeeded",
=======
        status: 501,
        body: {
          error: {
            type: "invalid_request",
            message: "Setup Mandate flow for Aci is not implemented",
            code: "IR_00",
          },
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
        },
      },
    },
    SaveCardUseNo3DSAutoCapture: {
      Request: {
        payment_method: "card",
        payment_method_data: {
          card: successfulNo3DSCardDetails,
        },
<<<<<<< HEAD
        currency: "EUR",
=======
        currency: "USD",
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
        setup_future_usage: "on_session",
        customer_acceptance: customerAcceptance,
      },
      Response: {
        status: 200,
        body: {
          status: "succeeded",
        },
      },
    },
<<<<<<< HEAD
    SaveCardUseNo3DSManualCapture: {
      Request: {
        payment_method: "card",
        payment_method_data: {
          card: successfulNo3DSCardDetails,
        },
        currency: "EUR",
        setup_future_usage: "on_session",
        customer_acceptance: customerAcceptance,
      },
      Response: {
        status: 200,
        body: {
          status: "requires_capture",
        },
      },
    },
=======
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
    SaveCardUseNo3DSAutoCaptureOffSession: {
      Configs: {
        TRIGGER_SKIP: true,
      },
      Request: {
        payment_method: "card",
        payment_method_type: "debit",
        payment_method_data: {
          card: successfulNo3DSCardDetails,
        },
        setup_future_usage: "off_session",
        customer_acceptance: customerAcceptance,
      },
      Response: {
        status: 200,
        body: {
          status: "succeeded",
        },
      },
    },
<<<<<<< HEAD
    SaveCardUseNo3DSManualCaptureOffSession: {
      Configs: {
        TRIGGER_SKIP: true,
      },
      Request: {
        payment_method: "card",
        payment_method_data: {
          card: successfulNo3DSCardDetails,
        },
        setup_future_usage: "off_session",
        customer_acceptance: customerAcceptance,
      },
      Response: {
        status: 200,
        body: {
          status: "requires_capture",
        },
      },
    },
=======
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
    SaveCardUse3DSAutoCaptureOffSession: {
      Configs: {
        TRIGGER_SKIP: true,
      },
      Request: {
        payment_method: "card",
        payment_method_type: "debit",
        payment_method_data: {
          card: successfulThreeDSTestCardDetails,
        },
        setup_future_usage: "off_session",
        customer_acceptance: customerAcceptance,
      },
      Response: {
        status: 200,
        body: {
          status: "requires_customer_action",
        },
      },
    },
    SaveCardConfirmAutoCaptureOffSession: {
      Request: {
        setup_future_usage: "off_session",
      },
      Response: {
        status: 200,
        body: {
          status: "succeeded",
        },
      },
    },
    PaymentMethodIdMandateNo3DSAutoCapture: {
<<<<<<< HEAD
=======
      Configs: {
        TRIGGER_SKIP: true,
      },
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
      Request: {
        payment_method: "card",
        payment_method_data: {
          card: successfulNo3DSCardDetails,
        },
<<<<<<< HEAD
        currency: "EUR",
=======
        currency: "USD",
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
        mandate_data: null,
        customer_acceptance: customerAcceptance,
      },
      Response: {
        status: 200,
        body: {
          status: "succeeded",
        },
      },
    },
<<<<<<< HEAD
    PaymentMethodIdMandateNo3DSManualCapture: {
      Request: {
        payment_method: "card",
        payment_method_data: {
          card: successfulNo3DSCardDetails,
        },
        currency: "EUR",
        mandate_data: null,
        customer_acceptance: customerAcceptance,
      },
      Response: {
        status: 200,
        body: {
          status: "requires_capture",
        },
      },
    },
    PaymentMethodIdMandate3DSAutoCapture: {
=======
    PaymentMethodIdMandate3DSAutoCapture: {
      Configs: {
        TRIGGER_SKIP: true,
      },
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
      Request: {
        payment_method: "card",
        payment_method_data: {
          card: successfulThreeDSTestCardDetails,
        },
<<<<<<< HEAD
        currency: "EUR",
=======
        currency: "USD",
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
        mandate_data: null,
        authentication_type: "three_ds",
        customer_acceptance: customerAcceptance,
      },
      Response: {
        status: 200,
        body: {
<<<<<<< HEAD
          status: "requires_customer_action",
        },
      },
    },
    PaymentMethodIdMandate3DSManualCapture: {
      Request: {
        payment_method: "card",
        payment_method_data: {
          card: successfulThreeDSTestCardDetails,
        },
        currency: "EUR",
        mandate_data: null,
        authentication_type: "three_ds",
        customer_acceptance: customerAcceptance,
      },
      Response: {
        status: 200,
        body: {
          status: "requires_customer_action",
        },
      },
    },
    MITManualCapture: {
      Request: {
        currency: "EUR",
      },
      Response: {
        status: 200,
        body: {
          status: "requires_capture",
        },
      },
    },
    MITAutoCapture: {
      Request: {
        currency: "EUR",
      },
      Response: {
        status: 200,
        body: {
=======
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
          status: "succeeded",
        },
      },
    },
    No3DSFailPayment: {
<<<<<<< HEAD
      Configs: {
        TRIGGER_SKIP: true,
      },
=======
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
      Request: {
        payment_method: "card",
        payment_method_data: {
          card: failedNo3DSCardDetails,
        },
        customer_acceptance: null,
        setup_future_usage: "on_session",
      },
      Response: {
        status: 200,
        body: {
          status: "failed",
<<<<<<< HEAD
          error_code: "100.390.112",
          error_message: "Technical Error in 3D system",
=======
          error_code: "200.300.404",
          error_message:
            "Field is card.expiryYear and the message is must match ^[0-9]{4}$",
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
          unified_code: "UE_9000",
          unified_message: "Something went wrong",
        },
      },
    },
  },
  bank_redirect_pm: {
    Ideal: {
      Request: {
        payment_method: "bank_redirect",
        payment_method_type: "ideal",
        payment_method_data: {
          bank_redirect: {
            ideal: {
              bank_name: "ing",
            },
          },
        },
        billing: billingAddress,
      },
      Response: {
        status: 200,
        body: {
          status: "requires_customer_action",
        },
      },
    },
    Przelewy24: {
      Request: {
        payment_method: "bank_redirect",
        payment_method_type: "przelewy24",
        payment_method_data: {
          bank_redirect: {
            przelewy24: {
              bank_name: "citi",
              billing_details: {
                email: "guest@juspay.in",
              },
            },
          },
        },
      },
      Response: {
        status: 200,
        body: {
          status: "failed",
          error_code: "200.100.103",
          error_message:
            "invalid Request Message. The request contains structural errors",
        },
      },
    },
  },
};
