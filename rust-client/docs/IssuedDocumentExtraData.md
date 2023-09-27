# IssuedDocumentExtraData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**show_sofort_button** | Option<**bool**> |  | [optional]
**multifatture_sent** | Option<**i32**> |  | [optional]
**ts_communication** | Option<**bool**> | Send issued document to \"Sistema Tessera Sanitaria\" | [optional]
**ts_flag_tipo_spesa** | Option<**f32**> | Issued document ts \"tipo spesa\" [TK, FC, FV, SV,SP, AD, AS, ECG, SR] | [optional]
**ts_pagamento_tracciato** | Option<**bool**> | Issued document ts traced payment | [optional]
**ts_tipo_spesa** | Option<**String**> | Can be [ 'TK', 'FC', 'FV', 'SV', 'SP', 'AD', 'AS', 'SR', 'CT', 'PI', 'IC', 'AA' ]. Refer to the technical specifications to learn more. | [optional]
**ts_opposizione** | Option<**bool**> | Issued document ts \"opposizione\" | [optional]
**ts_status** | Option<**i32**> | Issued document ts status | [optional]
**ts_file_id** | Option<**String**> | Issued document ts file id | [optional]
**ts_sent_date** | Option<[**String**](string.md)> | Issued document ts sent date | [optional]
**ts_full_amount** | Option<**bool**> | Issued document ts total amount | [optional]
**imported_by** | Option<**String**> | Issued document imported by software | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


