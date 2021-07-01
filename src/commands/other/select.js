const BaseCommand = require("../../utils/structures/BaseCommand");
const { MessageSelectMenu, MessageEmbed, MessageActionRow } = require("discord.js");

module.exports = class Select extends BaseCommand {
    constructor() {
        super("select", "other", ["sel"]);
    }

    async run(client, message, args) {
        const row = new MessageActionRow()
            .addComponents(
                new MessageSelectMenu()
                    .setCustomID('select')
                    .setPlaceholder('Nothing selected')
                    .addOptions([
                        {
                            label: 'Select me',
                            description: 'This is a description',
                            value: 'first_option',
                        },
                        {
                            label: 'You can select me too',
                            description: 'This is also a description',
                            value: 'second_option',
                        },
                        {
                            label: 'me too',
                            description: 'This is also another description',
                            value: 'third_option',
                        },
                    ]),
            );

        const embed = new MessageEmbed()
            .setColor("#0099ff")
            .setTitle("Oh Look!")
            .setURL("https://processorbot.xyz/")
            .setDescription("Selections! What do they do?");

        await message.channel.send({ content: "Hi there!", embeds: [embed], components: [row] });
    }
}