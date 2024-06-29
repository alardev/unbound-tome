has_role(user: User, role_key: String, campaign: Campaign) if
  role in user.roles and
  role.role_key = role_key and
  role.resource_table = "campaigns" and
  role.resource_id = campaign.id;

# Any logged-in user can create a new campaign.
has_permission(_: User, "create", _: Campaign);

resource Campaign {
    permissions = [
        # Update details about a Campaign
        "update",
        # Delete a Campaign
        "delete",
        # Create, update, and delete any Characters for a Campaign
        "manage_characters",
        # Grant or revoke Profile Roles for a Campaign
        "manage_roles"
    ];
    roles = [
        # Able to update a Campaign and manage Characters
        "manager",
        # Able to fully control a Campaign
        "admin"
    ];

    "update" if "manager";
    "manage_characters" if "manager";

    "delete" if "admin";
    "manage_roles" if "admin";
    "manager" if "admin";
}