const { MessageEmbed } = require("discord.js");

module.exports = {
    data: {
        "name": "kick",
        "description": "Kick a user",
        "options": [
            {
                "type": 6,
                "name": "user",
                "description": "Selected user",
                "required": true
            },
            {
                "type": 3,
                "name": "reason",
                "description": "Reason for kick",
                "required": true
            }
        ]
    },
    async run(client, interaction) {
        const user = interaction.options.getUser("user");
        const reason = interaction.options.getString("reason");

        if (!interaction.memberPermissions.has("KICK_MEMBERS")) {
            return interaction.reply({ content: "You need the `Kick Members` permission to kick a member.", ephemeral: true });
        }

        if (!interaction.guild.me.permissions.has("KICK_MEMBERS")) {
            return interaction.reply({ content: "I need the `Kick Members` permission to kick a member." });
        }

        interaction.guild.members.kick(user.id, { reason: reason });

        let kickEmbed = new MessageEmbed()
            .setDescription(`Kicked ${user} ${reason ? `for **${reason}**` : ""}`)
            .setColor("RED");
        interaction.reply({ embeds: [kickEmbed] });
    }
};