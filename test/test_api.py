"""
API Tests for areuok-server

This module contains comprehensive tests for all API endpoints:
- Device Management (register, get, signin, status)
- Supervision Management (request, pending, accept, reject, list, remove)

Requirements:
    pip install requests pytest

Usage:
    pytest test_api.py -v
    pytest test_api.py -v -k "test_device"  # Run only device tests
    pytest test_api.py -v -k "test_supervision"  # Run only supervision tests
"""

import uuid
import pytest
import requests
from typing import Optional
from dataclasses import dataclass


# Configuration
BASE_URL = "http://localhost:3000"


@dataclass
class Device:
    """Represents a registered device."""
    device_id: str
    device_name: str
    mode: str
    created_at: Optional[str] = None
    last_seen_at: Optional[str] = None


@dataclass
class SupervisionRequest:
    """Represents a supervision request."""
    request_id: str
    supervisor_id: str
    target_id: str
    status: str
    created_at: Optional[str] = None


@dataclass
class SupervisionRelation:
    """Represents an established supervision relation."""
    relation_id: str
    supervisor_id: str
    target_id: str
    supervisor_name: Optional[str] = None
    target_name: Optional[str] = None
    created_at: Optional[str] = None


class APIClient:
    """Client for interacting with the areuok-server API."""

    def __init__(self, base_url: str = BASE_URL):
        self.base_url = base_url
        self.session = requests.Session()
        self.session.headers.update({"Content-Type": "application/json"})

    def register_device(self, device_name: str, mode: str = "signin") -> requests.Response:
        """Register a new device."""
        return self.session.post(
            f"{self.base_url}/devices/register",
            json={"device_name": device_name, "mode": mode}
        )

    def get_device(self, device_id: str) -> requests.Response:
        """Get device information by ID."""
        return self.session.get(f"{self.base_url}/devices/{device_id}")

    def signin_device(self, device_id: str) -> requests.Response:
        """Sign in a device."""
        return self.session.post(f"{self.base_url}/devices/{device_id}/signin")

    def get_device_status(self, device_id: str) -> requests.Response:
        """Get device status including signin streak."""
        return self.session.get(f"{self.base_url}/devices/{device_id}/status")

    def create_supervision_request(self, supervisor_id: str, target_id: str) -> requests.Response:
        """Create a supervision request from supervisor to target."""
        return self.session.post(
            f"{self.base_url}/supervision/request",
            json={"supervisor_id": supervisor_id, "target_id": target_id}
        )

    def get_pending_requests(self, device_id: str) -> requests.Response:
        """Get pending supervision requests for a device."""
        return self.session.get(f"{self.base_url}/supervision/pending/{device_id}")

    def accept_supervision(self, supervisor_id: str, target_id: str) -> requests.Response:
        """Accept a supervision request."""
        return self.session.post(
            f"{self.base_url}/supervision/accept",
            json={"supervisor_id": supervisor_id, "target_id": target_id}
        )

    def reject_supervision(self, supervisor_id: str, target_id: str) -> requests.Response:
        """Reject a supervision request."""
        return self.session.post(
            f"{self.base_url}/supervision/reject",
            json={"supervisor_id": supervisor_id, "target_id": target_id}
        )

    def list_supervision_relations(self, device_id: str) -> requests.Response:
        """List all supervision relations for a device."""
        return self.session.get(f"{self.base_url}/supervision/list/{device_id}")

    def remove_supervision(self, relation_id: str) -> requests.Response:
        """Remove a supervision relation."""
        return self.session.delete(f"{self.base_url}/supervision/{relation_id}")

    def search_devices(self, query: str) -> requests.Response:
        """Search devices by name."""
        return self.session.get(f"{self.base_url}/search/devices", params={"q": query})


@pytest.fixture
def client():
    """Create an API client for testing."""
    return APIClient()


@pytest.fixture
def registered_device(client: APIClient):
    """Create and return a registered signin device."""
    response = client.register_device("Test Device", "signin")
    assert response.status_code == 200
    return Device(**response.json())


@pytest.fixture
def supervisor_device(client: APIClient):
    """Create and return a registered supervisor device."""
    response = client.register_device("Supervisor Device", "supervisor")
    assert response.status_code == 200
    return Device(**response.json())


@pytest.fixture
def target_device(client: APIClient):
    """Create and return a registered target (signin) device."""
    response = client.register_device("Target Device", "signin")
    assert response.status_code == 200
    return Device(**response.json())


class TestDeviceRegistration:
    """Tests for device registration endpoint."""

    def test_register_device_signin_mode(self, client: APIClient):
        """Test registering a device in signin mode."""
        response = client.register_device("My Phone", "signin")

        assert response.status_code == 200
        data = response.json()
        assert "device_id" in data
        assert data["device_name"] == "My Phone"
        assert data["mode"] == "signin"
        assert "created_at" in data
        assert "last_seen_at" in data

    def test_register_device_supervisor_mode(self, client: APIClient):
        """Test registering a device in supervisor mode."""
        response = client.register_device("Parent Phone", "supervisor")

        assert response.status_code == 200
        data = response.json()
        assert data["device_name"] == "Parent Phone"
        assert data["mode"] == "supervisor"

    def test_register_device_empty_name(self, client: APIClient):
        """Test registering a device with empty name."""
        response = client.register_device("", "signin")
        # Empty name should still work (depends on server validation)
        assert response.status_code in [200, 400, 422]

    def test_register_multiple_devices(self, client: APIClient):
        """Test registering multiple devices."""
        response1 = client.register_device("Device 1", "signin")
        response2 = client.register_device("Device 2", "supervisor")

        assert response1.status_code == 200
        assert response2.status_code == 200

        data1 = response1.json()
        data2 = response2.json()

        # Each device should have a unique ID
        assert data1["device_id"] != data2["device_id"]


class TestGetDevice:
    """Tests for get device endpoint."""

    def test_get_existing_device(self, client: APIClient, registered_device: Device):
        """Test getting an existing device."""
        response = client.get_device(registered_device.device_id)

        assert response.status_code == 200
        data = response.json()
        assert data["device_id"] == registered_device.device_id
        assert data["device_name"] == registered_device.device_name

    def test_get_nonexistent_device(self, client: APIClient):
        """Test getting a device that doesn't exist."""
        fake_id = "00000000-0000-0000-0000-000000000000"
        response = client.get_device(fake_id)

        assert response.status_code == 404

    def test_get_device_invalid_uuid(self, client: APIClient):
        """Test getting a device with an invalid UUID format."""
        response = client.get_device("not-a-valid-uuid")

        assert response.status_code in [400, 404, 422]


class TestSearchDevices:
    """Tests for search devices endpoint."""

    def test_search_devices_by_name(self, client: APIClient):
        """Test searching devices by name."""
        # Register a device with a unique name
        unique_name = f"SearchTest-{uuid.uuid4().hex[:8]}"
        reg_response = client.register_device(unique_name)
        assert reg_response.status_code == 200
        registered = reg_response.json()

        # Search for the device
        response = client.search_devices(unique_name[:10])
        assert response.status_code == 200
        data = response.json()
        assert isinstance(data, list)
        assert len(data) >= 1
        assert any(d["device_id"] == registered["device_id"] for d in data)

    def test_search_devices_partial_match(self, client: APIClient):
        """Test searching devices with partial name."""
        unique_prefix = f"Partial-{uuid.uuid4().hex[:6]}"
        client.register_device(f"{unique_prefix}-Device1")
        client.register_device(f"{unique_prefix}-Device2")

        response = client.search_devices(unique_prefix)
        assert response.status_code == 200
        data = response.json()
        assert len(data) >= 2

    def test_search_devices_empty_query(self, client: APIClient):
        """Test searching with empty query returns empty list."""
        response = client.search_devices("")
        assert response.status_code == 200
        data = response.json()
        assert data == []

    def test_search_devices_short_query(self, client: APIClient):
        """Test searching with single character returns empty list."""
        response = client.search_devices("a")
        assert response.status_code == 200
        data = response.json()
        assert data == []

    def test_search_devices_no_match(self, client: APIClient):
        """Test searching for non-existent device."""
        response = client.search_devices("nonexistent-device-xyz-12345")
        assert response.status_code == 200
        data = response.json()
        assert data == []


class TestDeviceSignin:
    """Tests for device signin endpoint."""

    def test_signin_device(self, client: APIClient, registered_device: Device):
        """Test signing in a device."""
        response = client.signin_device(registered_device.device_id)

        assert response.status_code == 200
        data = response.json()
        assert data["device_id"] == registered_device.device_id
        assert "date" in data
        assert data["streak"] >= 1

    def test_signin_device_twice_same_day(self, client: APIClient, registered_device: Device):
        """Test signing in a device twice on the same day."""
        response1 = client.signin_device(registered_device.device_id)
        response2 = client.signin_device(registered_device.device_id)

        assert response1.status_code == 200
        assert response2.status_code == 200

        data1 = response1.json()
        data2 = response2.json()

        # Streak should remain the same for same day signin
        assert data1["streak"] == data2["streak"]

    def test_signin_nonexistent_device(self, client: APIClient):
        """Test signing in a device that doesn't exist."""
        fake_id = "00000000-0000-0000-0000-000000000000"
        response = client.signin_device(fake_id)

        # Server might return 404 or create the signin anyway
        assert response.status_code in [200, 404, 500]


class TestDeviceStatus:
    """Tests for device status endpoint."""

    def test_get_status_new_device(self, client: APIClient, registered_device: Device):
        """Test getting status of a newly registered device."""
        response = client.get_device_status(registered_device.device_id)

        assert response.status_code == 200
        data = response.json()
        assert data["device_id"] == registered_device.device_id
        assert data["device_name"] == registered_device.device_name
        assert "streak" in data

    def test_get_status_after_signin(self, client: APIClient, registered_device: Device):
        """Test getting status after signing in."""
        # First signin
        client.signin_device(registered_device.device_id)

        # Then check status
        response = client.get_device_status(registered_device.device_id)

        assert response.status_code == 200
        data = response.json()
        assert data["streak"] >= 1
        assert data["last_signin"] is not None

    def test_get_status_nonexistent_device(self, client: APIClient):
        """Test getting status of a device that doesn't exist."""
        fake_id = "00000000-0000-0000-0000-000000000000"
        response = client.get_device_status(fake_id)

        assert response.status_code == 404


class TestSupervisionRequest:
    """Tests for supervision request endpoint."""

    def test_create_supervision_request(
        self, client: APIClient, supervisor_device: Device, target_device: Device
    ):
        """Test creating a supervision request."""
        response = client.create_supervision_request(
            supervisor_device.device_id, target_device.device_id
        )

        assert response.status_code == 200
        data = response.json()
        assert "request_id" in data
        assert data["supervisor_id"] == supervisor_device.device_id
        assert data["target_id"] == target_device.device_id
        assert data["status"] == "pending"

    def test_create_duplicate_supervision_request(
        self, client: APIClient, supervisor_device: Device, target_device: Device
    ):
        """Test creating a duplicate supervision request."""
        response1 = client.create_supervision_request(
            supervisor_device.device_id, target_device.device_id
        )
        response2 = client.create_supervision_request(
            supervisor_device.device_id, target_device.device_id
        )

        assert response1.status_code == 200
        # Duplicate might succeed with new request or fail
        assert response2.status_code in [200, 400, 409]


class TestPendingRequests:
    """Tests for pending supervision requests endpoint."""

    def test_get_pending_requests_empty(self, client: APIClient, target_device: Device):
        """Test getting pending requests when there are none."""
        response = client.get_pending_requests(target_device.device_id)

        assert response.status_code == 200
        data = response.json()
        assert isinstance(data, list)

    def test_get_pending_requests_with_request(
        self, client: APIClient, supervisor_device: Device, target_device: Device
    ):
        """Test getting pending requests when there is one."""
        # Create a supervision request
        client.create_supervision_request(
            supervisor_device.device_id, target_device.device_id
        )

        # Get pending requests for target
        response = client.get_pending_requests(target_device.device_id)

        assert response.status_code == 200
        data = response.json()
        assert isinstance(data, list)
        assert len(data) >= 1

        # Find our request
        matching_requests = [
            r for r in data if r["supervisor_id"] == supervisor_device.device_id
        ]
        assert len(matching_requests) >= 1
        assert matching_requests[0]["status"] == "pending"


class TestAcceptSupervision:
    """Tests for accept supervision endpoint."""

    def test_accept_supervision_request(
        self, client: APIClient, supervisor_device: Device, target_device: Device
    ):
        """Test accepting a supervision request."""
        # Create a supervision request
        client.create_supervision_request(
            supervisor_device.device_id, target_device.device_id
        )

        # Accept the request
        response = client.accept_supervision(
            supervisor_device.device_id, target_device.device_id
        )

        assert response.status_code == 200

        # Verify the relation was created
        relations_response = client.list_supervision_relations(supervisor_device.device_id)
        assert relations_response.status_code == 200
        relations = relations_response.json()
        assert len(relations) >= 1

    def test_accept_nonexistent_request(
        self, client: APIClient, supervisor_device: Device, target_device: Device
    ):
        """Test accepting a request that doesn't exist."""
        response = client.accept_supervision(
            supervisor_device.device_id, target_device.device_id
        )

        assert response.status_code == 404


class TestRejectSupervision:
    """Tests for reject supervision endpoint."""

    def test_reject_supervision_request(
        self, client: APIClient, supervisor_device: Device, target_device: Device
    ):
        """Test rejecting a supervision request."""
        # Create a supervision request
        client.create_supervision_request(
            supervisor_device.device_id, target_device.device_id
        )

        # Reject the request
        response = client.reject_supervision(
            supervisor_device.device_id, target_device.device_id
        )

        assert response.status_code == 200

        # Verify request is no longer pending
        pending_response = client.get_pending_requests(target_device.device_id)
        pending = pending_response.json()
        matching = [
            r for r in pending if r["supervisor_id"] == supervisor_device.device_id
        ]
        assert len(matching) == 0


class TestListSupervisionRelations:
    """Tests for list supervision relations endpoint."""

    def test_list_relations_empty(self, client: APIClient, registered_device: Device):
        """Test listing relations when there are none."""
        response = client.list_supervision_relations(registered_device.device_id)

        assert response.status_code == 200
        data = response.json()
        assert isinstance(data, list)

    def test_list_relations_as_supervisor(
        self, client: APIClient, supervisor_device: Device, target_device: Device
    ):
        """Test listing relations as a supervisor."""
        # Create and accept supervision
        client.create_supervision_request(
            supervisor_device.device_id, target_device.device_id
        )
        client.accept_supervision(
            supervisor_device.device_id, target_device.device_id
        )

        # List relations for supervisor
        response = client.list_supervision_relations(supervisor_device.device_id)

        assert response.status_code == 200
        data = response.json()
        assert len(data) >= 1

        relation = data[0]
        assert "relation_id" in relation
        assert "supervisor_id" in relation
        assert "target_id" in relation

    def test_list_relations_as_target(
        self, client: APIClient, supervisor_device: Device, target_device: Device
    ):
        """Test listing relations as a target device."""
        # Create and accept supervision
        client.create_supervision_request(
            supervisor_device.device_id, target_device.device_id
        )
        client.accept_supervision(
            supervisor_device.device_id, target_device.device_id
        )

        # List relations for target
        response = client.list_supervision_relations(target_device.device_id)

        assert response.status_code == 200
        data = response.json()
        assert len(data) >= 1


class TestRemoveSupervision:
    """Tests for remove supervision endpoint."""

    def test_remove_supervision_relation(
        self, client: APIClient, supervisor_device: Device, target_device: Device
    ):
        """Test removing a supervision relation."""
        # Create and accept supervision
        client.create_supervision_request(
            supervisor_device.device_id, target_device.device_id
        )
        client.accept_supervision(
            supervisor_device.device_id, target_device.device_id
        )

        # Get the relation ID
        relations_response = client.list_supervision_relations(supervisor_device.device_id)
        relations = relations_response.json()
        relation_id = relations[0]["relation_id"]

        # Remove the relation
        response = client.remove_supervision(relation_id)

        assert response.status_code == 200

        # Verify relation is removed
        relations_after = client.list_supervision_relations(supervisor_device.device_id).json()
        matching = [r for r in relations_after if r["relation_id"] == relation_id]
        assert len(matching) == 0

    def test_remove_nonexistent_relation(self, client: APIClient):
        """Test removing a relation that doesn't exist."""
        fake_id = "00000000-0000-0000-0000-000000000000"
        response = client.remove_supervision(fake_id)

        # Server might return 200 (no-op) or 404
        assert response.status_code in [200, 404]


class TestIntegrationWorkflow:
    """Integration tests for complete supervision workflow."""

    def test_complete_supervision_workflow(self, client: APIClient):
        """Test the complete supervision workflow from registration to monitoring."""
        # Step 1: Register a supervisor device
        supervisor_response = client.register_device("Parent Phone", "supervisor")
        assert supervisor_response.status_code == 200
        supervisor = supervisor_response.json()
        supervisor_id = supervisor["device_id"]

        # Step 2: Register a signin device
        target_response = client.register_device("Child Phone", "signin")
        assert target_response.status_code == 200
        target = target_response.json()
        target_id = target["device_id"]

        # Step 3: Supervisor creates supervision request
        request_response = client.create_supervision_request(supervisor_id, target_id)
        assert request_response.status_code == 200

        # Step 4: Target checks pending requests
        pending_response = client.get_pending_requests(target_id)
        assert pending_response.status_code == 200
        pending = pending_response.json()
        assert len(pending) >= 1

        # Step 5: Target accepts the request
        accept_response = client.accept_supervision(supervisor_id, target_id)
        assert accept_response.status_code == 200

        # Step 6: Verify supervision relation exists
        relations_response = client.list_supervision_relations(supervisor_id)
        assert relations_response.status_code == 200
        relations = relations_response.json()
        assert len(relations) >= 1

        # Step 7: Target signs in
        signin_response = client.signin_device(target_id)
        assert signin_response.status_code == 200

        # Step 8: Supervisor checks target status
        status_response = client.get_device_status(target_id)
        assert status_response.status_code == 200
        status = status_response.json()
        assert status["streak"] >= 1

        # Step 9: Remove supervision relation
        relation_id = relations[0]["relation_id"]
        remove_response = client.remove_supervision(relation_id)
        assert remove_response.status_code == 200

        # Verify relation is removed
        final_relations = client.list_supervision_relations(supervisor_id).json()
        matching = [r for r in final_relations if r["relation_id"] == relation_id]
        assert len(matching) == 0

    def test_reject_workflow(self, client: APIClient):
        """Test the workflow where a supervision request is rejected."""
        # Register devices
        supervisor = client.register_device("Supervisor", "supervisor").json()
        target = client.register_device("Target", "signin").json()

        # Create request
        client.create_supervision_request(supervisor["device_id"], target["device_id"])

        # Reject the request
        reject_response = client.reject_supervision(
            supervisor["device_id"], target["device_id"]
        )
        assert reject_response.status_code == 200

        # Verify no pending requests remain
        pending = client.get_pending_requests(target["device_id"]).json()
        matching = [
            r for r in pending if r["supervisor_id"] == supervisor["device_id"]
        ]
        assert len(matching) == 0

        # Verify no relations were created
        relations = client.list_supervision_relations(supervisor["device_id"]).json()
        matching_relations = [
            r for r in relations if r["target_id"] == target["device_id"]
        ]
        assert len(matching_relations) == 0


class TestHealthCheck:
    """Tests for server availability."""

    def test_server_is_running(self, client: APIClient):
        """Test that the server is responding."""
        # Try to register a device as a health check
        response = client.register_device("Health Check", "signin")
        assert response.status_code == 200


if __name__ == "__main__":
    # Run tests with pytest
    pytest.main([__file__, "-v"])
