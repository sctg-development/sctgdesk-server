import type { Messages } from '@/i18n'

/** Messages of the administration views (users, groups, address books). */
export default {
  "admin": {
    "update": "Update",
    "fields": {
      "id": "Id",
      "alias": "Alias",
      "hostname": "Hostname",
      "login": "Login",
      "platform": "Platform",
      "username": "Username",
      "password": "Password",
      "confirm": "Confirm",
      "email": "Email",
      "administrator": "Administrator",
      "group": "Group",
      "guid": "Guid",
      "name": "Name",
      "note": "Note",
      "status": "Status",
      "admin": "Admin",
      "user": "User",
      "rights": "Rights"
    },
    "users": {
      "add": "Add user",
      "edit": "Edit user",
      "active": "Active",
      "inactive": "Inactive",
      "activate": "Activate",
      "deactivate": "Deactivate",
      "copied": "Copied !",
      "fillAll": "Please fill all fields",
      "mismatch": "Password and confirm password do not match",
      "addFailed": "Failed to add user",
      "updateFailed": "Failed to update user",
      "activateFailed": "Failed to activate user {name}",
      "deactivateFailed": "Failed to deactivate user {name}",
      "confirmDelete": "Are you sure you want to delete this user?",
      "deleted": "User deleted successfully",
      "deleteFailed": "Failed to delete user"
    },
    "groups": {
      "add": "Add group",
      "update": "Update group",
      "name": "Group name",
      "confirmDelete": "Are you sure you want to delete this group?",
      "canConnectTo": "Can connect to",
      "canBeReachedBy": "Can be reached by"
    },
    "ab": {
      "personal": "Personal address book",
      "createShared": "Create a shared address book",
      "addShared": "Add shared address book",
      "sharedName": "Shared address book name",
      "update": "Update address book",
      "name": "Address book name",
      "viewRules": "View rules",
      "addRule": "Add rule",
      "edit": "Edit address book",
      "delete": "Delete address book",
      "confirmDelete": "Are you sure you want to delete this address book?",
      "rulesTitle": "Rules",
      "addRuleFor": "Add rule for {ab}",
      "nullOption": "Null",
      "ruleError": "User must be null for a group rule, group must be null for a user rule",
      "rights": {
        "none": "None",
        "read": "Read",
        "write": "Write",
        "full": "Full Control"
      },
      "viewRights": {
        "none": "None",
        "read": "Read",
        "readWrite": "Read/Write",
        "total": "Total Control",
        "unknown": "Unknown"
      }
    }
  }
} as Messages
