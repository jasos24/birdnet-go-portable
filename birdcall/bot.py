import discord
from discord.ext import commands
from discord import app_commands

from .log import logger
from .personality import (
    pick, JOIN_PUNS, STOP_PUNS, STATUS_ON_PUNS, STATUS_OFF_PUNS,
    RECONNECT_PUNS, SETRTMP_PUNS, PING_PUNS
)
from .ffmpeg_sender import start_recording, stop_recording
from .watchdog import watchdog

active_ffmpeg = None

intents = discord.Intents.default()
intents.voice_states = True
intents.guilds = True

bot = commands.Bot(command_prefix="!", intents=intents)
tree = bot.tree

@bot.event
async def on_ready():
    logger.info(f"BirdCall online as {bot.user}")
    bot.loop.create_task(watchdog(bot))
    await tree.sync()

@tree.command(name="join")
async def join(interaction: discord.Interaction):
    global active_ffmpeg

    user = interaction.user
    if not user.voice or not user.voice.channel:
        await interaction.response.send_message("Join a voice channel first.", ephemeral=True)
        return

    channel = user.voice.channel
    await interaction.response.defer(ephemeral=True)

    vc = interaction.guild.voice_client
    if vc:
        await vc.move_to(channel)
    else:
        vc = await channel.connect()

    active_ffmpeg = await start_recording(vc)

    await interaction.followup.send(pick(JOIN_PUNS), ephemeral=True)

@tree.command(name="stop")
async def stop(interaction: discord.Interaction):
    global active_ffmpeg

    vc = interaction.guild.voice_client
    if vc:
        await stop_recording(vc, active_ffmpeg)
        await vc.disconnect()

    active_ffmpeg = None

    await interaction.response.send_message(pick(STOP_PUNS), ephemeral=True)

@tree.command(name="status")
async def status(interaction: discord.Interaction):
    vc = interaction.guild.voice_client
    msg = pick(STATUS_ON_PUNS) if vc and vc.is_connected() else pick(STATUS_OFF_PUNS)
    await interaction.response.send_message(msg, ephemeral=True)

@tree.command(name="reconnect")
async def reconnect(interaction: discord.Interaction):
    await interaction.response.send_message(pick(RECONNECT_PUNS), ephemeral=True)

@tree.command(name="setrtmp")
async def setrtmp(interaction: discord.Interaction, url: str):
    await interaction.response.send_message(pick(SETRTMP_PUNS), ephemeral=True)

@tree.command(name="ping")
async def ping(interaction: discord.Interaction):
    await interaction.response.send_message(pick(PING_PUNS), ephemeral=True)
