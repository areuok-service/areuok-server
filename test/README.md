============== test session starts ===============
platform darwin -- Python 3.13.11, pytest-9.0.2, pluggy-1.6.0
collected 28 items

test_api.py::TestDeviceRegistration::test_register_device_signin_mode PASSED [  3%]
test_api.py::TestDeviceRegistration::test_register_device_supervisor_mode PASSED [  7%]
test_api.py::TestDeviceRegistration::test_register_device_empty_name PASSED [ 10%]
test_api.py::TestDeviceRegistration::test_register_multiple_devices PASSED [ 14%]
test_api.py::TestGetDevice::test_get_existing_device PASSED [ 17%]
test_api.py::TestGetDevice::test_get_nonexistent_device PASSED [ 21%]
test_api.py::TestGetDevice::test_get_invalid_uuid PASSED [ 25%]
test_api.py::TestSigninDevice::test_signin_device PASSED [ 28%]
test_api.py::TestSigninDevice::test_signin_device_twice_same_day PASSED [ 32%]
test_api.py::TestSigninDevice::test_signin_nonexistent_device PASSED [ 35%]
test_api.py::TestDeviceStatus::test_get_status_new_device PASSED [ 39%]
test_api.py::TestDeviceStatus::test_get_status_after_signin PASSED [ 42%]
test_api.py::TestDeviceStatus::test_get_status_nonexistent_device PASSED [ 46%]
test_api.py::TestSupervisionRequest::test_create_supervision_request PASSED [ 50%]
test_api.py::TestSupervisionRequest::test_create_duplicate_supervision_request PASSED [ 53%]
test_api.py::TestPendingRequests::test_get_pending_requests_empty PASSED [ 57%]
test_api.py::TestPendingRequests::test_get_pending_requests_with_request PASSED [ 60%]
test_api.py::TestAcceptSupervision::test_accept_supervision_request PASSED [ 64%]
test_api.py::TestAcceptSupervision::test_accept_nonexistent_request PASSED [ 67%]
test_api.py::TestRejectSupervision::test_reject_supervision_request PASSED [ 71%]
test_api.py::TestListSupervisionRelations::test_list_relations_empty PASSED [ 75%]
test_api.py::TestListSupervisionRelations::test_list_relations_as_supervisor PASSED [ 78%]
test_api.py::TestListSupervisionRelations::test_list_relations_as_target PASSED [ 82%]
test_api.py::TestRemoveSupervision::test_remove_supervision_relation PASSED [ 85%]
test_api.py::TestRemoveSupervision::test_remove_nonexistent_relation PASSED [ 89%]
test_api.py::TestIntegrationWorkflow::test_complete_supervision_workflow PASSED [ 92%]
test_api.py::TestIntegrationWorkflow::test_reject_workflow PASSED [ 96%]
test_api.py::TestHealthCheck::test_server_is_running PASSED [100%]

=============== 28 passed in 1.50s ===============
```

## 📦 Prerequisites

### Required Software

- **Python**: 3.13 or later
  - Check version: `python3 --version`
  - Download: https://www.python.org/downloads/
  
- **uv**: Python package manager (recommended)
  - Install: `curl -LsSf https://astral.sh/uv/install.sh | sh`
  - Alternative: pip (included with Python)

### Required Services

- **AreUOK Server**: Running on `http://localhost:3000`
  - Must be started before running tests
  - See main README.md for setup instructions

## 🚀 Quick Start

### Fastest Way to Run Tests

```bash
# 1. Start the server (from project root)
cd ..
./start-docker.sh

# 2. Run all tests
cd test
uv run pytest -v

# 3. Done! All 28 tests should pass
```

### Alternative: Using pip

```bash
# 1. Start the server
cd ..
docker-compose up -d

# 2. Install dependencies
cd test
pip install -r pyproject.toml

# 3. Run tests
pytest -v
```

## 🔧 Installation

### Using uv (Recommended)

```bash
cd test

# Install dependencies
uv sync

# Activate virtual environment (optional)
source .venv/bin/activate  # Linux/Mac
# or
.venv\Scripts\activate     # Windows
```

### Using pip

```bash
cd test

# Create virtual environment
python3 -m venv .venv

# Activate virtual environment
source .venv/bin/activate  # Linux/Mac
# or
.venv\Scripts\activate     # Windows

# Install dependencies
pip install -r pyproject.toml
```

### Verify Installation

```bash
# Check pytest is installed
pytest --version

# Should show: pytest 9.0.2 or similar
```

## 🧪 Running Tests

### Basic Commands

```bash
# Run all tests
uv run pytest

# Run with verbose output
uv run pytest -v

# Run with very verbose output
uv run pytest -vv

# Show detailed print statements
uv run pytest -vv -s
```

### Running Specific Tests

```bash
# Run a specific test class
uv run pytest -v test_api.py::TestDeviceRegistration

# Run a specific test method
uv run pytest -v test_api.py::TestDeviceRegistration::test_register_device_signin_mode

# Run tests matching a pattern
uv run pytest -v -k "device"
uv run pytest -v -k "supervision"
uv run pytest -v -k "signin"
uv run pytest -v -k "pending"

# Run integration tests only
uv run pytest -v -k "integration"
```

### Running Tests by Category

```bash
# Device management tests
uv run pytest -v -k "test_device or test_register or test_get or test_signin or test_status"

# Supervision management tests
uv run pytest -v -k "supervision"

# Error handling tests
uv run pytest -v -k "nonexistent or invalid or empty"

# Happy path tests
uv run pytest -v -k "not (nonexistent or invalid or empty or duplicate)"
```

### Advanced Options

```bash
# Stop on first failure
uv run pytest -x

# Run failed tests only
uv run pytest --lf

# Run tests in parallel (requires pytest-xdist)
uv run pytest -n auto

# Generate coverage report
uv run pytest --cov=test_api --cov-report=html

# Run with custom server URL
BASE_URL=http://localhost:8080 uv run pytest

# Run with timeout
uv run pytest --timeout=10
```

## 📁 Test Structure

### File Organization

```
test/
├── test_api.py              # Main test file (all tests)
├── README.md                 # This file
├── pyproject.toml            # Python dependencies
├── .python-version           # Python version specification
├── .venv/                    # Virtual environment (generated)
└── .pytest_cache/           # Pytest cache (generated)
```

### Test Class Hierarchy

```python
# Data Models
Device
SupervisionRequest
SupervisionRelation

# API Client
APIClient
├── register_device()
├── get_device()
├── signin_device()
├── get_device_status()
├── create_supervision_request()
├── get_pending_requests()
├── accept_supervision()
├── reject_supervision()
├── list_supervision_relations()
└── remove_supervision()

# Test Fixtures
client()          # APIClient instance
registered_device() # Sign-in mode device
supervisor_device()  # Supervisor mode device
target_device()   # Target device for supervision

# Test Classes
TestDeviceRegistration
TestGetDevice
TestSigninDevice
TestDeviceStatus
TestSupervisionRequest
TestPendingRequests
TestAcceptSupervision
TestRejectSupervision
TestListSupervisionRelations
TestRemoveSupervision
TestIntegrationWorkflow
TestHealthCheck
```

### Test Naming Convention

```python
# Format: test_<action>_<resource>_<condition>
test_register_device_signin_mode()
test_get_existing_device()
test_signin_device_twice_same_day()
test_accept_supervision_request()
```

## 📖 Test Examples

### Example 1: Device Registration

```python
class TestDeviceRegistration:
    def test_register_device_signin_mode(self, client: APIClient):
        # Register a device with "signin" mode
        device = client.register_device("test-device", "signin")
        
        # Verify device was created
        assert device.device_id is not None
        assert device.device_name == "test-device"
        assert device.mode == "signin"
        assert device.created_at is not None
```

### Example 2: Device Sign-in with Streak

```python
class TestSigninDevice:
    def test_signin_device(self, client: APIClient, registered_device):
        # Sign in the device
        signin = client.signin_device(registered_device.device_id)
        
        # Verify sign-in was recorded
        assert signin.device_id == registered_device.device_id
        assert signin.streak == 1  # First sign-in
        assert signin.date is not None
```

### Example 3: Supervision Workflow

```python
class TestIntegrationWorkflow:
    def test_complete_supervision_workflow(self, client: APIClient):
        # 1. Create supervisor and target devices
        supervisor = client.register_device("supervisor", "supervisor")
        target = client.register_device("target", "signin")
        
        # 2. Create supervision request
        request = client.create_supervision_request(
            supervisor.device_id,
            target.device_id
        )
        assert request.status == "pending"
        
        # 3. Get pending requests for target
        pending = client.get_pending_requests(target.device_id)
        assert len(pending) == 1
        assert pending[0].request_id == request.request_id
        
        # 4. Accept supervision request
        client.accept_supervision(supervisor.device_id, target.device_id)
        
        # 5. Verify supervision relation exists
        relations = client.list_supervision_relations(supervisor.device_id)
        assert len(relations) == 1
        assert relations[0].supervisor_id == supervisor.device_id
        assert relations[0].target_id == target.device_id
```

### Example 4: Error Handling

```python
class TestGetDevice:
    def test_get_nonexistent_device(self, client: APIClient):
        # Try to get a device that doesn't exist
        fake_id = uuid.uuid4()
        
        with pytest.raises(HTTPError) as exc_info:
            client.get_device(fake_id)
        
        # Verify 404 error
        assert exc_info.value.response.status_code == 404
```

## 🔍 Troubleshooting

### Common Issues and Solutions

#### 1. Connection Refused Error

**Problem**:
```
requests.exceptions.ConnectionError: Connection refused
```

**Solution**:
```bash
# Check if server is running
curl http://localhost:3000/health

# Start server if not running
cd ..
docker-compose up -d

# Wait for server to be ready
docker-compose logs -f server
```

#### 2. Import Errors

**Problem**:
```
ModuleNotFoundError: No module named 'pytest'
```

**Solution**:
```bash
# Reinstall dependencies
cd test
rm -rf .venv
uv sync

# Or with pip
pip install -r pyproject.toml
```

#### 3. Timeout Errors

**Problem**:
```
requests.exceptions.Timeout: Request timed out
```

**Solution**:
```bash
# Check server logs
cd ..
docker-compose logs server

# Restart server
docker-compose restart server

# Increase timeout in tests
# Modify TIMEOUT constant in test_api.py
```

#### 4. Database Errors

**Problem**:
```
500 Internal Server Error: Database error
```

**Solution**:
```bash
# Check database is running
docker-compose ps postgres

# Reset database
docker-compose down -v
docker-compose up -d

# Check migrations
docker-compose logs server | grep migration
```

#### 5. UUID Validation Errors

**Problem**:
```
400 Bad Request: Invalid UUID
```

**Solution**:
```python
# Ensure you're using valid UUIDs
import uuid
device_id = uuid.uuid4()

# Or use the device_id returned from API
device = client.register_device("test", "signin")
device_id = device.device_id  # Use this ID
```

#### 6. Environment Variable Issues

**Problem**:
```
BASE_URL not set or incorrect
```

**Solution**:
```bash
# Set custom BASE_URL
export BASE_URL=http://localhost:8080
uv run pytest

# Or inline
BASE_URL=http://localhost:8080 uv run pytest
```

### Debug Mode

Enable verbose output for debugging:

```bash
# Very verbose with print statements
uv run pytest -vv -s

# Stop on first failure with full traceback
uv run pytest -vv -x --tb=long

# Run specific test with debugging
uv run pytest -vv -s test_api.py::TestDeviceRegistration::test_register_device_signin_mode
```

### Test Database Cleanup

Tests create real data in the database. To clean up:

```bash
# Option 1: Reset entire database
cd ..
docker-compose down -v
docker-compose up -d

# Option 2: Manual cleanup
docker-compose exec postgres psql -U postgres -d areuok -c "
  TRUNCATE TABLE signin_records CASCADE;
  TRUNCATE TABLE supervision_relations CASCADE;
  TRUNCATE TABLE supervision_requests CASCADE;
  TRUNCATE TABLE devices CASCADE;
"
```

## ✍️ Writing New Tests

### Template for New Tests

```python
import pytest
from test_api import APIClient

class TestNewFeature:
    """Tests for new feature"""
    
    def test_new_functionality_success(self, client: APIClient):
        """
        Test that new feature works correctly
        """
        # Arrange: Set up test data
        device = client.register_device("test-device", "signin")
        
        # Act: Call the API
        result = client.your_new_endpoint(device.device_id)
        
        # Assert: Verify results
        assert result is not None
        assert result.some_field == "expected_value"
    
    def test_new_functionality_error_case(self, client: APIClient):
        """
        Test error handling for new feature
        """
        # Arrange
        invalid_id = "invalid-uuid"
        
        # Act & Assert
        with pytest.raises(HTTPError) as exc_info:
            client.your_new_endpoint(invalid_id)
        
        assert exc_info.value.response.status_code == 400
```

### Adding Fixtures

```python
@pytest.fixture
def custom_device(client: APIClient):
    """Create a device with custom settings"""
    device = client.register_device("custom-device", "supervisor")
    # Add custom setup
    yield device
    # Cleanup if needed
    client.remove_supervision(device.device_id)
```

### Test Categories

1. **Happy Path Tests**: Normal successful operations
2. **Error Tests**: Invalid inputs, missing data, etc.
3. **Edge Cases**: Boundary conditions, empty inputs, etc.
4. **Integration Tests**: Multi-step workflows
5. **Performance Tests**: Load, stress, timing (optional)

### Best Practices

```python
# DO: Use descriptive names
def test_register_device_with_valid_data_should_succeed(self, client):
    pass

# DON'T: Use vague names
def test_it_works(self, client):
    pass

# DO: Arrange, Act, Assert pattern
def test_signin_increments_streak(self, client, registered_device):
    # Arrange
    initial_streak = 0
    
    # Act
    signin = client.signin_device(registered_device.device_id)
    
    # Assert
    assert signin.streak > initial_streak

# DO: Use fixtures for shared setup
def test_with_fixtures(self, client, registered_device, supervisor_device):
    pass

# DON'T: Repeat setup code in every test
def test_without_fixtures(self, client):
    device = client.register_device("test", "signin")
    supervisor = client.register_device("sup", "supervisor")
    # ... test code
```

## 📚 Best Practices

### Test Organization

1. **One test class per API resource group**
2. **One test method per specific scenario**
3. **Use fixtures for common setup**
4. **Keep tests independent and isolated**

### Test Data Management

1. **Use unique names for each test**
2. **Clean up after tests if needed**
3. **Don't rely on specific database state**
4. **Use UUIDs for unique identifiers**

### Error Testing

```python
# Always test error cases
def test_signin_nonexistent_device(self, client: APIClient):
    fake_id = uuid.uuid4()
    
    with pytest.raises(HTTPError) as exc_info:
        client.signin_device(fake_id)
    
    assert exc_info.value.response.status_code == 404
```

### Assertions

```python
# Use specific assertions
assert device.device_id is not None
assert device.mode == "signin"
assert len(relations) == 1

# Use assertion messages for clarity
assert response.status_code == 200, f"Expected 200, got {response.status_code}"

# Use pytest.raises for expected exceptions
with pytest.raises(HTTPError) as exc_info:
    risky_operation()
assert exc_info.value.response.status_code == 400
```

### Documentation

```python
def test_complete_supervision_workflow(self, client: APIClient):
    """
    Test the complete supervision workflow:
    1. Create supervisor and target devices
    2. Send supervision request
    3. Target accepts request
    4. Verify supervision relation exists
    
    This is an integration test that verifies multiple
    API endpoints work together correctly.
    """
    pass
```

## 🔄 Continuous Integration

### GitHub Actions Example

```yaml
name: Run Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    
    services:
      postgres:
        image: postgres:16
        env:
          POSTGRES_USER: postgres
          POSTGRES_PASSWORD: postgres
          POSTGRES_DB: areuok
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
        ports:
          - 5432:5432
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Set up Python
        uses: actions/setup-python@v4
        with:
          python-version: '3.13'
      
      - name: Install uv
        run: |
          curl -LsSf https://astral.sh/uv/install.sh | sh
          echo "$HOME/.cargo/bin" >> $GITHUB_PATH
      
      - name: Install dependencies
        working-directory: ./test
        run: uv sync
      
      - name: Start server
        run: |
          cd ..
          docker-compose up -d
          sleep 10
      
      - name: Run tests
        working-directory: ./test
        run: uv run pytest -v
      
      - name: Upload test results
        if: always()
        uses: actions/upload-artifact@v3
        with:
          name: test-results
          path: test/pytest-report.html
```

### Running Tests in CI

```bash
# In CI environment
export CI=true

# Run tests with coverage
uv run pytest --cov=test_api --cov-report=xml --cov-report=html

# Generate JUnit report for CI
uv run pytest --junitxml=pytest-report.xml
```

## 📞 Support

For issues with the test suite:

1. Check this README for common issues
2. Review test output for specific error messages
3. Ensure server is running properly
4. Verify all prerequisites are installed

## 📄 License

Same as the main project.

## 🤝 Contributing

When adding new features:

1. Write tests for the new feature
2. Ensure all existing tests still pass
3. Update this README if needed
4. Document any new test fixtures

---

**Last Updated**: 2024-01-14  
**Test Suite Version**: 1.0.0  
**Python Version**: 3.13+  
**Total Tests**: 28  
