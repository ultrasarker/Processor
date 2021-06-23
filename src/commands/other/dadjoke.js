const fetch = require('node-fetch');
const BaseCommand = require('../../utils/structures/BaseCommand');

module.exports = class DadJoke extends BaseCommand {
    constructor() {
        super('dadjoke', 'other', ['dad', 'djoke']);
    }

    async run(client, message, args) {
        fetch('https://icanhazdadjoke.com/slack')
            .then(res => res.json())
            .then(json => {
                message.channel.send({ content: json.attachments[0].text });
            });
    }
}