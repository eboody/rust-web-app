# AdSetSchedule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**start_minute** | Option<**i32**> | A 0 based minute of the day representing when the schedule starts | [optional]
**end_minute** | **i32** | A 0 based minute of the day representing when the schedule ends | 
**days** | **Vec<i32>** | Array of ints representing which days the schedule is active. Valid values are 0-6 with 0 representing Sunday,  1 representing Monday, ... and 6 representing Saturday.  | 
**timezone_type** | Option<**String**> | Timezone type | [optional][default to User]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


