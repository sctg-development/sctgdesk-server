import type { Messages } from '@/i18n'

/** Messages of the administration views (users, groups, address books). */
export default {
  "admin": {
    "update": "Aggiorna",
    "fields": {
      "id": "Id",
      "alias": "Alias",
      "hostname": "Nome host",
      "login": "Login",
      "platform": "Piattaforma",
      "username": "Nome utente",
      "password": "Password",
      "confirm": "Conferma",
      "email": "Email",
      "administrator": "Amministratore",
      "group": "Gruppo",
      "guid": "Guid",
      "name": "Nome",
      "note": "Nota",
      "status": "Stato",
      "admin": "Admin",
      "user": "Utente",
      "rights": "Diritti"
    },
    "users": {
      "add": "Aggiungi utente",
      "edit": "Modifica utente",
      "active": "Attivo",
      "inactive": "Inattivo",
      "activate": "Attiva",
      "deactivate": "Disattiva",
      "copied": "Copiato!",
      "fillAll": "Compila tutti i campi",
      "mismatch": "La password e la conferma non coincidono",
      "addFailed": "Impossibile aggiungere l'utente",
      "updateFailed": "Impossibile aggiornare l'utente",
      "activateFailed": "Impossibile attivare l'utente {name}",
      "deactivateFailed": "Impossibile disattivare l'utente {name}",
      "confirmDelete": "Vuoi davvero eliminare questo utente?",
      "deleted": "Utente eliminato con successo",
      "deleteFailed": "Impossibile eliminare l'utente"
    },
    "groups": {
      "add": "Aggiungi gruppo",
      "update": "Aggiorna gruppo",
      "name": "Nome del gruppo",
      "confirmDelete": "Vuoi davvero eliminare questo gruppo?",
      "canConnectTo": "Può connettersi a",
      "canBeReachedBy": "Può essere raggiunto da"
    },
    "ab": {
      "personal": "Rubrica personale",
      "createShared": "Crea una rubrica condivisa",
      "addShared": "Aggiungi rubrica condivisa",
      "sharedName": "Nome della rubrica condivisa",
      "update": "Aggiorna rubrica",
      "name": "Nome della rubrica",
      "viewRules": "Visualizza regole",
      "addRule": "Aggiungi regola",
      "edit": "Modifica rubrica",
      "delete": "Elimina rubrica",
      "confirmDelete": "Vuoi davvero eliminare questa rubrica?",
      "rulesTitle": "Regole",
      "addRuleFor": "Aggiungi regola per {ab}",
      "nullOption": "Nessuno",
      "ruleError": "L'utente deve essere vuoto in una regola di gruppo e il gruppo vuoto in una regola utente",
      "rights": {
        "none": "Nessuno",
        "read": "Lettura",
        "write": "Scrittura",
        "full": "Controllo completo"
      },
      "viewRights": {
        "none": "Nessuno",
        "read": "Lettura",
        "readWrite": "Lettura/scrittura",
        "total": "Controllo completo",
        "unknown": "Sconosciuto"
      }
    }
  }
} as Messages
