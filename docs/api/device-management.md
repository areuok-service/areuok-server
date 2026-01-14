# Device Management API

## Register Device

Register a new device with a specific mode. Device names must be unique across all devices.

### Endpoint

```
POST /devices/register
```

### Request Headers

```
Content-Type: application/json
```

### Request Body

```json
{
  "device_name": "string",
  "imei": "string (optional)",
  "mode": "signin|supervisor"
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| device_name | string | Yes | Name of device (max 255 characters, must be unique) |
| imei | string | No | Device IMEI for binding (optional, binds to existing device if IMEI matches) |
| mode | string | Yes | Device mode: "signin" or "supervisor" |

### Response

**Status Code**: `200 OK`

```json
{
  "device_id": "550e8400-e29b-41d4-a716-446655440000",
  "device_name": "My Phone",
  "imei": "123456789012345",
  "mode": "signin",
  "created_at": "2024-01-14T13:00:00.000000Z",
  "last_seen_at": "2024-01-14T13:00:00.000000Z",
  "last_name_updated_at": "2024-01-14T13:00:00.000000Z"
}
```

### Example

```bash
curl -X POST http://localhost:3000/devices/register \
  -H "Content-Type: application/json" \
  -d '{
    "device_name": "My Phone",
    "imei": "123456789012345",
    "mode": "signin"
  }'
```

### Behavior

- **Device Name Uniqueness**: Device names must be unique across all devices. If a name is already in use, registration will fail.
- **IMEI Binding**: If an IMEI is provided and matches an existing device, the existing device will be returned instead of creating a new one.
- **New Device**: If no IMEI is provided or IMEI doesn't match any device, a new device will be created.

### Error Responses

- `400 Bad Request` - Invalid device name, mode, or duplicate name
  ```json
  {
    "error": "Device name already exists"
  }
  ```

## Update Device Name

Update the device name. Device name must be unique and can only be changed once every 15 days.

### Endpoint

```
PATCH /devices/{id}/name
```

### Path Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| id | UUID | Yes | Device UUID |

### Request Headers

```
Content-Type: application/json
```

### Request Body

```json
{
  "device_name": "string"
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| device_name | string | Yes | New device name (max 255 characters, must be unique) |

### Response

**Status Code**: `200 OK`

```json
{
  "device_id": "550e8400-e29b-41d4-a716-446655440000",
  "device_name": "My New Phone",
  "imei": "123456789012345",
  "mode": "signin",
  "created_at": "2024-01-14T13:00:00.000000Z",
  "last_seen_at": "2024-01-14T13:00:00.000000Z",
  "last_name_updated_at": "2024-01-29T13:00:00.000000Z"
}
```

### Example

```bash
curl -X PATCH http://localhost:3000/devices/550e8400-e29b-41d4-a716-446655440000/name \
  -H "Content-Type: application/json" \
  -d '{
    "device_name": "My New Phone"
  }'
```

### Behavior

- **15-Day Cooldown**: Device names can only be updated once every 15 days.
- **Name Uniqueness**: The new name must be unique across all devices.
- **First Update**: If the device has never updated its name before, it can be changed immediately.

### Error Responses

- `400 Bad Request` - Name already exists or cooldown period not met
  ```json
  {
    "error": "Device name already exists"
  }
  ```
  ```json
  {
    "error": "Device name cannot be updated. Last updated 10 days ago. Minimum 15 days required."
  }
  ```

- `404 Not Found` - Device not found
  ```json
  {
    "error": "Device not found"
  }
  ```

## Get Device

Retrieve information about a specific device.

### Endpoint

```
GET /devices/{id}
```

### Path Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| id | UUID | Yes | Device UUID |

### Response

**Status Code**: `200 OK`

```json
{
  "device_id": "550e8400-e29b-41d4-a716-446655440000",
  "device_name": "My Phone",
  "imei": "123456789012345",
  "mode": "signin",
  "created_at": "2024-01-14T13:00:00.000000Z",
  "last_seen_at": "2024-01-14T13:00:00.000000Z",
  "last_name_updated_at": "2024-01-14T13:00:00.000000Z"
}
```

### Example

```bash
curl http://localhost:3000/devices/550e8400-e29b-41d4-a716-446655440000
```

### Error Responses

- `404 Not Found` - Device not found
  ```json
  {
    "error": "Device not found"
  }
  ```

## Search Devices

Search devices by name to get their UUID. This is useful for finding devices to establish supervision relationships.

### Endpoint

```
GET /search/devices
```

### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| q | string | Yes | Search query (minimum 2 characters) |

### Response

**Status Code**: `200 OK`

```json
[
  {
    "device_id": "550e8400-e29b-41d4-a716-446655440000",
    "device_name": "My Phone",
    "imei": "123456789012345",
    "mode": "signin",
    "created_at": "2024-01-14T13:00:00.000000Z",
    "last_seen_at": "2024-01-14T13:00:00.000000Z",
    "last_name_updated_at": "2024-01-14T13:00:00.000000Z"
  }
]
```

### Example

```bash
curl "http://localhost:3000/search/devices?q=phone"
```

### Behavior

- Returns up to 20 matching devices
- Search is case-insensitive
- Matches partial device names
- Returns empty array if query is less than 2 characters

---

## Sign In Device

Record a sign-in for a device and update streak.

### Endpoint

```
POST /devices/{id}/signin
```

### Path Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| id | UUID | Yes | Device UUID |

### Response

**Status Code**: `200 OK`

```json
{
  "device_id": "550e8400-e29b-41d4-a716-446655440000",
  "date": "2024-01-14",
  "streak": 5
}
```

| Field | Type | Description |
|-------|------|-------------|
| device_id | UUID | Device that signed in |
| date | string | Date of sign-in (ISO 8601) |
| streak | integer | Current sign-in streak |

### Example

```bash
curl -X POST http://localhost:3000/devices/550e8400-e29b-41d4-a716-446655440000/signin
```

### Behavior

- First sign-in of day: Creates new record with streak = 1
- Subsequent sign-ins same day: No change to streak
- Sign-in after missed day: Resets streak to 1
- Consecutive daily sign-ins: Increments streak

### Error Responses

- `404 Not Found` - Device not found
  ```json
  {
    "error": "Device not found"
  }
  ```

## Get Device Status

Get sign-in status of a device (for supervisors).

### Endpoint

```
GET /devices/{id}/status
```

### Path Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| id | UUID | Yes | Device UUID |

### Response

**Status Code**: `200 OK`

```json
{
  "device_id": "550e8400-e29b-41d4-a716-446655440000",
  "device_name": "My Phone",
  "mode": "signin",
  "last_seen_at": "2024-01-14T13:00:00.000000Z",
  "status": {
    "signed_in_today": true,
    "streak": 5,
    "last_signin_date": "2024-01-14"
  }
}
```

| Field | Type | Description |
|-------|------|-------------|
| signed_in_today | boolean | Whether device signed in today |
| streak | integer | Current sign-in streak |
| last_signin_date | string | Date of last sign-in (ISO 8601) |

### Example

```bash
curl http://localhost:3000/devices/550e8400-e29b-41d4-a716-446655440000/status
```

### Error Responses

- `404 Not Found` - Device not found
  ```json
  {
    "error": "Device not found"
  }
  ```
