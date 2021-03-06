# List all sources

List all sources for a workspace. The list of sources can be paginated using the `page` and `size` query parameters.

**URL** : `/workspaces/<workspace_id>/sources`

**Method** : `GET`

## Query Parameters

**page** : The index of the page to retrieve. Defaults to 0.
**size** : The number of sources per page. Defaults to 20.

## Success Response

The response contains a list of sources of a workspace.

**Code** : `200 OK`

**Content examples**

```json
{
  "status": "success",
  "data": [
    "total": 1,
    "data": [
      {
        "id": 1,
        "type": "youtube_video",
        "term": "https://youtube.com/watch?v=54aef32",
        "tags": [
          {"label": "incident_code", "description": null}
        ]
      }  
    ]
  ]
}
```
