const ytdl = require('ytdl-core');
const search = require('youtube-search');
const PREFIX = process.env.PREFIX;
const opts = {
    maxResults: 1,
    key: process.env.YOUTUBE_TOKEN,
    type: 'audio'
};

let servers = [];
let songIndex = 0;
module.exports = {
    run: async (client, message, args) => {
        if(args.startsWith(`${PREFIX}p`)) {
            return message.channel.send(":grey_question: **Please choose a title or URL of a song to play.**")
            .then(msg => {
                msg.delete({timeout: 4000});
            });
        }
        let results = await search(args, opts).catch(err => console.log(err));
        const voiceChannel = message.member.voice.channel;
        try {
            const permissions = voiceChannel.permissionsFor(message.client.user);
            if (!permissions.has('CONNECT')) {
                message.channel.send(":x: **I don't have permissions to connect to the voice channel.**")
                .then(msg => {
                    msg.delete({timeout: 4000});
                });
                return;
            }
            if (!permissions.has('SPEAK')) {
                message.channel.send(":x: **I don't have permissions to speak in the voice channel.**")
                .then(msg => {
                    msg.delete({timeout: 4000});
                });
                return;
            }
        }
        catch (err) {
            message.channel.send(':x: **I may not have proper permissions set to connect or play music in a channel.**')
        }
                if(!servers[message.guild.id]) servers[message.guild.id] = {
                    queue: []
                }

        var server = servers[message.guild.id];

        server.queue.push(args);

        function playNextSong() { 
            let server = servers[message.guild.id];
            
            server.dispatcher = connection.play(ytdl(server.queue[songIndex], {filter: "audioonly"}));
            server.queue.shift();

            server.dispatcher.on('finish', () => {
                if (songIndex + 1 < server.queue.length) {
                    playNextSong();
                }
                else {
                    connection.disconnect();
                }
                songIndex++;
            })
        }

        let songEmbed = {
            author: "Now Playing ",
            title: '',
            url: '',
            color: `RANDOM`,
            timestamp: new Date(),
            fields : [
                {
                    name: 'Publish Date',
                    value: '',
                    inline: true
                },
                {
                    name: 'Owner',
                    value: '',
                    inline: true
                },
                {
                    name: 'Total Views',
                    value: '',
                    inline: true
                },
                {
                    name: 'Duration',
                    value: '',
                    inline: true
                },
                {
                    name: ':thumbsup: Likes',
                    value: '',
                    inline: true
                },
                {
                    name: ':thumbsdown: Dislikes',
                    value: '',
                    inline: true
                }
            ]
        }

        try {
            var connection = await voiceChannel.join()
        } catch (err) {
            console.log(`There was an error connecting to the voice channel: ${err}`);
            return message.channel.send(`**There was an error connecting to the voice channel: ${err}**`);
        }
        try {
            if(results) {
                let youtubeResults = results.results;
                let titles = youtubeResults.map(result => {
                    result.title + "\n" + result.link;
                    server.queue.push(result.link);
                    server.queue.shift();
                    message.channel.send(':mag_right: **Searching for ** ' + '`' + args + '`')
                    const dispatcher = connection.play(ytdl(server.queue[songIndex])
                    .on('info', (info) => {
                        songEmbed.title = info.videoDetails.title;
                        songEmbed.url = info.videoDetails.video_url;
                        songEmbed.fields[0].value = info.videoDetails.publishDate;
                        songEmbed.fields[1].value = info.videoDetails.ownerChannelName;
                        songEmbed.fields[2].value = info.videoDetails.viewCount;
                        (info.videoDetails.lengthSeconds % 60) < 10 ? 
                        songEmbed.fields[3].value = `${Math.floor(info.videoDetails.lengthSeconds / 60)}:0${info.videoDetails.lengthSeconds % 60}` :
                        songEmbed.fields[3].value = `${Math.floor(info.videoDetails.lengthSeconds / 60)}:${info.videoDetails.lengthSeconds % 60}`;
                        songEmbed.fields[4].value = info.videoDetails.likes;
                        songEmbed.fields[5].value = info.videoDetails.dislikes;
                        message.channel.send({embed: songEmbed});
                    }))
                    .on('finish', () => {
                        if (songIndex + 1 < queue.length) {
                            playNextSong();
                        }
                    })
                });
            }
        playNextSong();  
        }
        catch (err) {
            message.delete();
            message.channel.send(`:x: Failed to play song: ${err}`)
            .then(msg => {
                msg.delete({timeout: 15000});
            });
        }    
    },
    aliases: ['p'],
    description: 'Plays a song'
}