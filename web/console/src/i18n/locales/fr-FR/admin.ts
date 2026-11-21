import type { Messages } from '@/i18n'

/** Messages of the administration views (users, groups, address books). */
export default {
  "admin": {
    "update": "Mettre à jour",
    "fields": {
      "id": "Id",
      "alias": "Alias",
      "hostname": "Nom d'hôte",
      "login": "Identifiant",
      "platform": "Plateforme",
      "username": "Nom d'utilisateur",
      "password": "Mot de passe",
      "confirm": "Confirmation",
      "email": "E-mail",
      "administrator": "Administrateur",
      "group": "Groupe",
      "guid": "Guid",
      "name": "Nom",
      "note": "Note",
      "status": "Statut",
      "admin": "Admin",
      "user": "Utilisateur",
      "rights": "Droits"
    },
    "users": {
      "add": "Ajouter un utilisateur",
      "edit": "Modifier l'utilisateur",
      "active": "Actif",
      "inactive": "Inactif",
      "activate": "Activer",
      "deactivate": "Désactiver",
      "copied": "Copié !",
      "fillAll": "Veuillez remplir tous les champs",
      "mismatch": "Le mot de passe et sa confirmation ne correspondent pas",
      "addFailed": "Impossible d'ajouter l'utilisateur",
      "updateFailed": "Impossible de mettre à jour l'utilisateur",
      "activateFailed": "Impossible d'activer l'utilisateur {name}",
      "deactivateFailed": "Impossible de désactiver l'utilisateur {name}",
      "confirmDelete": "Voulez-vous vraiment supprimer cet utilisateur ?",
      "deleted": "Utilisateur supprimé",
      "deleteFailed": "Impossible de supprimer l'utilisateur"
    },
    "groups": {
      "add": "Ajouter un groupe",
      "update": "Mettre à jour le groupe",
      "name": "Nom du groupe",
      "confirmDelete": "Voulez-vous vraiment supprimer ce groupe ?",
      "canConnectTo": "Peut se connecter à",
      "canBeReachedBy": "Peut être joint par"
    },
    "ab": {
      "personal": "Carnet d'adresses personnel",
      "createShared": "Créer un carnet d'adresses partagé",
      "addShared": "Ajouter un carnet d'adresses partagé",
      "sharedName": "Nom du carnet d'adresses partagé",
      "update": "Mettre à jour le carnet d'adresses",
      "name": "Nom du carnet d'adresses",
      "viewRules": "Voir les règles",
      "addRule": "Ajouter une règle",
      "edit": "Modifier le carnet d'adresses",
      "delete": "Supprimer le carnet d'adresses",
      "confirmDelete": "Voulez-vous vraiment supprimer ce carnet d'adresses ?",
      "rulesTitle": "Règles",
      "addRuleFor": "Ajouter une règle pour {ab}",
      "nullOption": "Aucun",
      "ruleError": "L'utilisateur doit être vide pour une règle de groupe, le groupe doit être vide pour une règle d'utilisateur",
      "rights": {
        "none": "Aucun",
        "read": "Lecture",
        "write": "Écriture",
        "full": "Contrôle total"
      },
      "viewRights": {
        "none": "Aucun",
        "read": "Lecture",
        "readWrite": "Lecture/écriture",
        "total": "Contrôle total",
        "unknown": "Inconnu"
      }
    }
  }
} as Messages
