# Entity1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | Entity id | [optional]
**code** | Option<**String**> | Entity code | [optional]
**name** | **String** | Entity name | 
**r#type** | Option<**String**> | Entity type | [optional]
**first_name** | Option<**String**> | Entity first name | [optional]
**last_name** | Option<**String**> | Entity last name | [optional]
**contact_person** | Option<**String**> | Entity contact person | [optional]
**vat_number** | Option<**String**> | Entity vat number | [optional]
**tax_code** | Option<**String**> | Entity tax code | [optional]
**address_street** | Option<**String**> | Entitity address street | [optional]
**address_postal_code** | Option<**String**> | Entity address postal code | [optional]
**address_city** | Option<**String**> | Entity address city | [optional]
**address_province** | Option<**String**> | Entity address province | [optional]
**address_extra** | Option<**String**> | Entity address extra info | [optional]
**country** | Option<**String**> | Entity country | [optional][default to Italia]
**country_iso** | Option<**String**> | Entity country iso code | [optional]
**email** | Option<**String**> | Entity email | [optional]
**certified_email** | Option<**String**> | Entity certified email | [optional]
**phone** | Option<**String**> | Entity phone | [optional]
**fax** | Option<**String**> | Entity fax | [optional]
**notes** | Option<**String**> | Entity extra | [optional]
**default_payment_terms** | Option<**i32**> | [Only for client] Client default payment terms | [optional]
**default_vat** | Option<[**models::VatType3**](VatType_3.md)> |  | [optional]
**default_payment_terms_type** | Option<**String**> | [Only for client] Client default payment terms type | [optional][default to Standard]
**default_payment_method** | Option<[**models::PaymentMethod3**](PaymentMethod_3.md)> |  | [optional]
**bank_name** | Option<**String**> | [Only for client] Client bank name | [optional]
**bank_iban** | Option<**String**> | [Only for client] Client bank iban | [optional]
**bank_swift_code** | Option<**String**> | [Only for client] Client bank swift code | [optional]
**shipping_address** | Option<**String**> | [Only for client] Client Shipping address | [optional]
**e_invoice** | Option<**bool**> | [Only for client] Use e-invoices. | [optional][default to false]
**ei_code** | Option<**String**> | [Only for client] E-invoices code. | [optional]
**has_intent_declaration** | Option<**bool**> | [Only for client] Has intent declaration. | [optional]
**intent_declaration_protocol_number** | Option<**String**> | [Only for client] Client intent declaration protocol number | [optional]
**intent_declaration_protocol_date** | Option<[**String**](string.md)> | [Only for client] Client intent declaration protocol date | [optional]
**created_at** | Option<**String**> | Entity creation date | [optional]
**updated_at** | Option<**String**> | Entity last update date | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


