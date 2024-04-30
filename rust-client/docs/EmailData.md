# EmailData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**recipient_email** | Option<**String**> | Email recipient | [optional]
**default_sender_email** | Option<[**models::EmailDataDefaultSenderEmail**](EmailData_default_sender_email.md)> |  | [optional]
**sender_emails_list** | Option<[**Vec<models::SenderEmail>**](SenderEmail.md)> | List of all emails from which the document can be sent | [optional]
**cc_email** | Option<**String**> | Email cc [by default is the logged company email] | [optional]
**subject** | Option<**String**> | Email subject | [optional]
**body** | Option<**String**> | Email body | [optional]
**document_exists** | Option<**bool**> | Document exists if it is not a delivery note | [optional]
**delivery_note_exists** | Option<**bool**> | Document is a delivery note | [optional]
**attachment_exists** | Option<**bool**> | Document has attachment | [optional]
**accompanying_invoice_exists** | Option<**bool**> | Document has accompanying invoice | [optional]
**default_attach_pdf** | Option<**bool**> | Attach document pdf | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


