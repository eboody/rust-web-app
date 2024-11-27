# Roles

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Unique identifier for the role. | [optional]
**name** | Option<**String**> | Name of the role. | [optional]
**icon** | Option<**String**> | The role's icon. | [optional]
**description** | Option<**String**> | Description of the role. | [optional]
**parent** | Option<[**models::RolesParent**](Roles_parent.md)> |  | [optional]
**children** | Option<[**Vec<models::RolesChildrenInner>**](Roles_children_inner.md)> | $t:field_options.directus_roles.children_note | [optional]
**policies** | Option<[**serde_json::Value**](.md)> |  | [optional]
**users** | Option<[**Vec<models::RolesUsersInner>**](Roles_users_inner.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


