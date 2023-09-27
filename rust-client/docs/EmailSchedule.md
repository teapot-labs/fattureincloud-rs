# EmailSchedule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sender_id** | Option<**i32**> | Email sender id [required if **sender_email** is not specified] | [optional]
**sender_email** | Option<**String**> | Email sender address [required if **sender_id** is not specified] | [optional]
**recipient_email** | **String** | Email recipient emails [comma separated] | 
**subject** | **String** | Email subject | 
**body** | **String** | Email body [HTML Escaped] [max size 50KiB] | 
**include** | [**crate::models::EmailScheduleInclude**](EmailSchedule_include.md) |  | 
**attach_pdf** | **bool** | Attach the pdf of the document | 
**send_copy** | **bool** | Send a copy of the email to the **cc_email** specified by **Get email data** | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


