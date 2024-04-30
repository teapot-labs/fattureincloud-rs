# Client

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | Client id | [optional]
**code** | Option<**String**> | Client code | [optional]
**name** | Option<**String**> | Client name | [optional]
**r#type** | Option<**String**> | Client type | [optional]
**first_name** | Option<**String**> | Client first name | [optional]
**last_name** | Option<**String**> | Client last name | [optional]
**contact_person** | Option<**String**> | Client contact person | [optional]
**vat_number** | Option<**String**> | Client vat number | [optional]
**tax_code** | Option<**String**> | Client tax code | [optional]
**address_street** | Option<**String**> | Client address street | [optional]
**address_postal_code** | Option<**String**> | Client address postal code | [optional]
**address_city** | Option<**String**> | Client address city | [optional]
**address_province** | Option<**String**> | Client address province | [optional]
**address_extra** | Option<**String**> | Client address extra info | [optional]
**country** | Option<**String**> | Client country | [optional]
**country_iso** | Option<**String**> | Client country iso code | [optional]
**email** | Option<**String**> | Client email | [optional]
**certified_email** | Option<**String**> | Client certified email | [optional]
**phone** | Option<**String**> | Client phone | [optional]
**fax** | Option<**String**> | Client fax | [optional]
**notes** | Option<**String**> | Client extra | [optional]
**default_vat** | Option<[**models::VatType**](VatType.md)> |  | [optional]
**default_payment_terms** | Option<**i32**> | Client default payment terms | [optional]
**default_payment_terms_type** | Option<**String**> | Payment terms type | [optional][default to Standard]
**default_payment_method** | Option<[**models::PaymentMethod**](PaymentMethod.md)> |  | [optional]
**bank_name** | Option<**String**> | Client bank name | [optional]
**bank_iban** | Option<**String**> | Client bank iban | [optional]
**bank_swift_code** | Option<**String**> | Client bank swift code | [optional]
**shipping_address** | Option<**String**> | Client shipping address | [optional]
**e_invoice** | Option<**bool**> | Use e-invoices for this entity | [optional]
**ei_code** | Option<**String**> | Client e-invoice code  | [optional]
**discount_highlight** | Option<**bool**> | Highlight Discount | [optional]
**default_discount** | Option<**f64**> | Client default discount | [optional]
**has_intent_declaration** | Option<**bool**> | Client has intent declaration | [optional]
**intent_declaration_protocol_number** | Option<**String**> | Client intent declaration protocol number | [optional]
**intent_declaration_protocol_date** | Option<[**String**](string.md)> | Client intent declaration protocol date | [optional]
**created_at** | Option<**String**> | Client creation date | [optional]
**updated_at** | Option<**String**> | Client last update date | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


