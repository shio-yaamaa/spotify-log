# git-commits-to-bq

Transfer logs already saved as commits in the [spotify-backup repository](https://github.com/shio-yaamaa/spotify-backup) to BigQuery.

## Execution

```sh
export GOOGLE_APPLICATION_CREDENTIALS="/Users/shiori/big-query-a8961d46abd7.json"
GCP_ACCESS_TOKEN="$(gcloud auth application-default print-access-token)" cargo run
```

Create three tables described below and define their schema before executing the program.

## Tables

### action

- timestamp: TIMESTAMP (REQUIRED)
- action_type: STRING (REQUIRED)
  - "addition" | "removal" | "transfer"
  - Modifications are not stored in this table
- source_playlist_id: STRING (NULLABLE)
- destination_playlist_id: STRING (NULLABLE)
- track_id: STRING (REQUIRED)

```json
[
  {
    "name": "timestamp",
    "type": "TIMESTAMP",
    "mode": "REQUIRED"
  },
  {
    "name": "action_type",
    "type": "STRING",
    "mode": "REQUIRED"
  },
  {
    "name": "source_playlist_id",
    "type": "STRING"
  },
  {
    "name": "destination_playlist_id",
    "type": "STRING"
  },
  {
    "name": "track_id",
    "type": "STRING",
    "mode": "REQUIRED"
  }
]
```

### track

- id: STRING
- name: STRING
- artist_ids: STRING[]

```json
[
  {
    "name": "id",
    "type": "STRING",
    "mode": "REQUIRED"
  },
  {
    "name": "name",
    "type": "STRING",
    "mode": "REQUIRED"
  },
  {
    "name": "artist_ids",
    "type": "STRING",
    "mode": "REPEATED"
  }
]
```

### artist

- id: STRING
- name: STRING

```json
[
  {
    "name": "id",
    "type": "STRING",
    "mode": "REQUIRED"
  },
  {
    "name": "name",
    "type": "STRING",
    "mode": "REQUIRED"
  }
]
```
