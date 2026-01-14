# Database Schema

## Overview

The areuok-server uses PostgreSQL as its database backend. The schema is managed through SQL migrations in the `crates/db/migrations` directory.

## Tables

### devices

Stores device information and IMEI binding data.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| device_id | UUID | PRIMARY KEY | Unique device identifier |
| device_name | VARCHAR(255) | NOT NULL, UNIQUE | Device display name (must be unique across all devices) |
| imei | VARCHAR(255) | UNIQUE, NULLABLE | Device IMEI for binding (optional) |
| mode | device_mode | NOT NULL | Device mode: 'signin' or 'supervisor' |
| created_at | TIMESTAMPTZ | NOT NULL | Device registration timestamp |
| last_seen_at | TIMESTAMPTZ | NOT NULL | Last activity timestamp |
| last_name_updated_at | TIMESTAMPTZ | NULLABLE | Last device name update timestamp |

**Indexes:**
- `idx_devices_imei` on `imei` column

**Constraints:**
- `devices_device_name_key` - UNIQUE constraint on `device_name`
- `devices_imei_key` - UNIQUE constraint on `imei` (allows NULL)

### supervision_requests

Stores pending supervision relationship requests.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| request_id | UUID | PRIMARY KEY | Unique request identifier |
| supervisor_id | UUID | NOT NULL, FK | ID of device requesting supervision |
| target_id | UUID | NOT NULL, FK | ID of device being supervised |
| status | supervision_status | NOT NULL | Request status: 'pending', 'accepted', 'rejected' |
| created_at | TIMESTAMPTZ | NOT NULL | Request creation timestamp |

**Indexes:**
- `idx_supervision_requests_target` on (target_id, status)
- `idx_supervision_requests_supervisor` on (supervisor_id)

**Foreign Keys:**
- `supervisor_id` → devices(device_id) ON DELETE CASCADE
- `target_id` → devices(device_id) ON DELETE CASCADE

### supervision_relations

Stores active supervision relationships.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| relation_id | UUID | PRIMARY KEY | Unique relationship identifier |
| supervisor_id | UUID | NOT NULL, FK | ID of supervising device |
| target_id | UUID | NOT NULL, FK | ID of supervised device |
| created_at | TIMESTAMPTZ | NOT NULL | Relationship establishment timestamp |

**Indexes:**
- `idx_supervision_relations_supervisor` on (supervisor_id)
- `idx_supervision_relations_target` on (target_id)

**Foreign Keys:**
- `supervisor_id` → devices(device_id) ON DELETE CASCADE
- `target_id` → devices(device_id) ON DELETE CASCADE

### signin_records

Tracks device sign-in history and streak information.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | SERIAL | PRIMARY KEY | Auto-incrementing record ID |
| device_id | UUID | NOT NULL, FK | Device that signed in |
| date | TIMESTAMPTZ | NOT NULL | Sign-in date |
| streak | INTEGER | NOT NULL | Current sign-in streak count |

**Indexes:**
- `idx_signin_records_device` on (device_id)

**Foreign Keys:**
- `device_id` → devices(device_id) ON DELETE CASCADE

**Unique Constraint:**
- `(device_id, date)` - Only one sign-in record per device per day

## Enums

### device_mode

Enumeration for device operating modes.

| Value | Description |
|--------|-------------|
| signin | Device can sign in and be supervised |
| supervisor | Device can supervise other devices |

### supervision_status

Enumeration for supervision request status.

| Value | Description |
|--------|-------------|
| pending | Request awaiting approval |
| accepted | Request approved, relationship active |
| rejected | Request declined |

## Business Rules

### Device Name Management

1. **Uniqueness Constraint**
   - Device names must be unique across all devices in the system
   - Enforced by database UNIQUE constraint `devices_device_name_key`

2. **Update Cooldown**
   - Device names can only be changed once every 15 days
   - Tracked via `last_name_updated_at` column
   - Server validates time difference before allowing updates
   - First name update has no cooldown period

3. **Validation Flow**
   ```
   User tries to update name:
   ├─ Check if name already exists in database
   ├─ Check if last_name_updated_at is NULL (first update)
   │  └─ Allow if NULL
   └─ Check if (NOW() - last_name_updated_at) >= 15 days
      └─ Allow if true, reject if false
   ```

### IMEI Device Binding

1. **Optional Field**
   - IMEI is stored in `devices.imei` column
   - Can be NULL for devices without IMEI binding
   - UNIQUE constraint allows multiple NULL values but unique non-NULL values

2. **Registration Behavior**
   ```
   POST /devices/register with IMEI:
   ├─ Query device by IMEI
   ├─ If device exists:
   │  └─ Return existing device (IMEI binding recovery)
   └─ If no device with that IMEI:
      └─ Create new device with IMEI
   ```

3. **Device Recovery**
   - If same IMEI is used for registration again, system retrieves existing device
   - Useful for recovering device accounts on same physical device
   - Maintains device identity across application reinstalls

4. **Uniqueness**
   - IMEI values must be unique across all devices (except NULL)
   - Enforced by database UNIQUE constraint `devices_imei_key`

## Migration History

| Migration | Description | Date |
|------------|-------------|------|
| `20240114_000000_initial.up.sql` | Initial schema creation | 2024-01-14 |
| `20260114_120000_add_device_imei.up.sql` | Added IMEI, last_name_updated_at, and name uniqueness constraints | 2026-01-14 |

## Running Migrations

### Docker Environment

```bash
docker-compose exec server cargo run --bin server
```

Migrations run automatically on server startup.

### Local Development

```bash
cd crates/db
sqlx migrate run --database-url postgresql://user:pass@localhost/dbname
```

## Database Backup and Restore

### Backup

```bash
docker-compose exec postgres pg_dump -U areuok areuok > backup.sql
```

### Restore

```bash
docker-compose exec -T postgres psql -U areuok areuok < backup.sql
```

## Performance Considerations

### Indexes

All foreign key columns and frequently queried columns are indexed:

- `device_id` in all tables (primary keys)
- `imei` in `devices` table (for IMEI binding lookups)
- `device_name` has unique constraint (implicitly indexed)
- `target_id` and `status` in `supervision_requests` (composite index)
- `device_id` in `signin_records` (for streak queries)

### Query Optimization

1. **IMEI Lookup**: Uses `idx_devices_imei` for fast IMEI-based device retrieval
2. **Name Search**: Uses implicit index from UNIQUE constraint on `device_name`
3. **Supervision Queries**: Composite index on `(target_id, status)` optimizes pending request lookups

## Data Retention

- **Device Records**: Retained indefinitely (soft delete via supervision cascade)
- **Sign-in Records**: Retained indefinitely for streak history
- **Supervision Requests**: Retained indefinitely
- **Supervision Relations**: Retained until explicitly deleted

## Cleanup Commands

### Remove All Data (Development Only)

```bash
docker-compose down -v
```

This removes all volumes and database data.

### Reset Specific Table

```sql
TRUNCATE TABLE signin_records CASCADE;
TRUNCATE TABLE supervision_relations CASCADE;
TRUNCATE TABLE supervision_requests CASCADE;
TRUNCATE TABLE devices CASCADE;
```
