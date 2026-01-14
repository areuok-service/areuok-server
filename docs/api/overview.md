# API Overview

## Base Information

- **Base URL**: `http://localhost:3000`
- **Protocol**: HTTP/1.1
- **Content-Type**: `application/json`
- **Character Set**: UTF-8

## Authentication

Currently, API does not require authentication. This may change in future versions.

## Common Response Codes

### Success Codes

- `200 OK` - Request succeeded
- `201 Created` - Resource created successfully

### Error Codes

- `400 Bad Request` - Invalid request data
  - Device name already exists
  - Device name update cooldown period not met (15 days)
- `404 Not Found` - Resource not found
- `500 Internal Server Error` - Server error

## Common Response Format

### Success Response
```json
{
  "data": { ... }
}
```

### Error Response
```json
{
  "error": {
    "message": "Error description",
    "code": "ERROR_CODE"
  }
}
```

## API Endpoints Summary

### Device Management

- `POST /devices/register` - Register a new device (supports IMEI binding)
- `GET /devices/{id}` - Get device information
- `PATCH /devices/{id}/name` - Update device name (15-day cooldown, unique names required)
- `GET /search/devices?q={query}` - Search devices by name to get UUID
- `POST /devices/{id}/signin` - Record device sign-in
- `GET /devices/{id}/status` - Get device sign-in status

### Supervision Management

- `POST /supervision/request` - Create supervision request
- `GET /supervision/pending/{id}` - Get pending requests
- `POST /supervision/accept` - Accept supervision request
- `POST /supervision/reject` - Reject supervision request
- `GET /supervision/list/{id}` - List supervision relations
- `DELETE /supervision/{relation_id}` - Remove supervision relation

## Data Types

### Device Mode

- `signin` - Device can sign in and be supervised
- `supervisor` - Device can supervise other devices

### Supervision Status

- `pending` - Request awaiting approval
- `accepted` - Request approved
- `rejected` - Request declined

## Device Name Management

### Device Name Constraints

- **Uniqueness**: Device names must be unique across all devices in the system
- **Length**: Maximum 255 characters
- **Update Cooldown**: Can only be changed once every 15 days
- **First Change**: No cooldown for first name update

### IMEI Binding

- **Optional**: IMEI is optional during registration
- **Binding**: If IMEI is provided and matches an existing device, the existing device is returned
- **Uniqueness**: IMEI values must be unique across all devices

## Pagination

Currently, pagination is not implemented. All endpoints return complete result sets.

## Rate Limiting

Currently, rate limiting is not implemented. Please make requests responsibly.

## CORS

The API supports CORS with default settings. Cross-origin requests are allowed.

## Versioning

Current API version: `v1.0.0`

Future versions will maintain backward compatibility where possible.

## Business Rules

### Device Name Updates

1. **First Registration**: Device name set during initial registration
2. **First Update**: Can be changed immediately with no restrictions
3. **Subsequent Updates**: 15-day cooldown period between updates
4. **Uniqueness**: New name must not be in use by any other device
5. **Validation**: Server validates both uniqueness and cooldown before allowing update

### IMEI Device Binding

1. **Registration with IMEI**: IMEI can optionally be provided during registration
2. **Device Recovery**: If same IMEI is used again, existing device is retrieved instead of creating new one
3. **Persistence**: IMEI is stored with device record for binding purposes
