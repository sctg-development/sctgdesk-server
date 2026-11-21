import type { Messages } from '@/i18n'

/** Messages of the administration views (users, groups, address books). */
export default {
  "admin": {
    "update": "Aktualisieren",
    "fields": {
      "id": "ID",
      "alias": "Alias",
      "hostname": "Hostname",
      "login": "Anmeldename",
      "platform": "Plattform",
      "username": "Benutzername",
      "password": "Passwort",
      "confirm": "Bestätigen",
      "email": "E-Mail",
      "administrator": "Administrator",
      "group": "Gruppe",
      "guid": "Guid",
      "name": "Name",
      "note": "Notiz",
      "status": "Status",
      "admin": "Admin",
      "user": "Benutzer",
      "rights": "Rechte"
    },
    "users": {
      "add": "Benutzer hinzufügen",
      "edit": "Benutzer bearbeiten",
      "active": "Aktiv",
      "inactive": "Inaktiv",
      "activate": "Aktivieren",
      "deactivate": "Deaktivieren",
      "copied": "Kopiert!",
      "fillAll": "Bitte alle Felder ausfüllen",
      "mismatch": "Passwort und Bestätigung stimmen nicht überein",
      "addFailed": "Benutzer konnte nicht hinzugefügt werden",
      "updateFailed": "Benutzer konnte nicht aktualisiert werden",
      "activateFailed": "Benutzer {name} konnte nicht aktiviert werden",
      "deactivateFailed": "Benutzer {name} konnte nicht deaktiviert werden",
      "confirmDelete": "Möchten Sie diesen Benutzer wirklich löschen?",
      "deleted": "Benutzer erfolgreich gelöscht",
      "deleteFailed": "Benutzer konnte nicht gelöscht werden"
    },
    "groups": {
      "add": "Gruppe hinzufügen",
      "update": "Gruppe aktualisieren",
      "name": "Gruppenname",
      "confirmDelete": "Möchten Sie diese Gruppe wirklich löschen?",
      "canConnectTo": "Darf sich verbinden mit",
      "canBeReachedBy": "Erreichbar für"
    },
    "ab": {
      "personal": "Persönliches Adressbuch",
      "createShared": "Gemeinsames Adressbuch erstellen",
      "addShared": "Gemeinsames Adressbuch hinzufügen",
      "sharedName": "Name des gemeinsamen Adressbuchs",
      "update": "Adressbuch aktualisieren",
      "name": "Name des Adressbuchs",
      "viewRules": "Regeln anzeigen",
      "addRule": "Regel hinzufügen",
      "edit": "Adressbuch bearbeiten",
      "delete": "Adressbuch löschen",
      "confirmDelete": "Möchten Sie dieses Adressbuch wirklich löschen?",
      "rulesTitle": "Regeln",
      "addRuleFor": "Regel hinzufügen für {ab}",
      "nullOption": "Keine",
      "ruleError": "Bei einer Gruppenregel muss der Benutzer leer sein, bei einer Benutzerregel die Gruppe",
      "rights": {
        "none": "Keine",
        "read": "Lesen",
        "write": "Schreiben",
        "full": "Vollzugriff"
      },
      "viewRights": {
        "none": "Keine",
        "read": "Lesen",
        "readWrite": "Lesen/Schreiben",
        "total": "Vollzugriff",
        "unknown": "Unbekannt"
      }
    }
  }
} as Messages
