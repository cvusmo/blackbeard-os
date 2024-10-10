from collections import defaultdict
import os
from datetime import datetime

def parse_tracker_file(file_path):
    """Parse the tracker file and return a list of game data."""
    games = []
    with open(file_path, 'r') as file:
        for line in file:
            # Parse each line to extract the result and opponent colors
            parts = line.strip().split(', ')
            result = 'win' if '1W' in parts[0] else 'loss'  # Check if '1W' indicates a win
            # Extract opponent colors from the game description
            opponent_colors = [part.split('vs ')[1].split(',')[0] for part in parts if 'vs ' in part]
            if opponent_colors:
                games.append({"opponent_colors": opponent_colors, "result": result})
    return games

# File path to the tracker data
tracker_file_path = '/home/echo/projects/mtg/mtga_standard_r/tracker.txt'  # Absolute path to the file

# Tracker game data
games = parse_tracker_file(tracker_file_path)
# print(f"Total games parsed: {len(games)}")  # Add this line


# Color combinations to groups
color_groups = {
    frozenset(["Azorius"]): "Azorius",
    frozenset(["Boros"]): "Boros",
    frozenset(["Dimir"]): "Dimir",
    frozenset(["Golgari"]): "Golgari",
    frozenset(["Gruul"]): "Gruul",
    frozenset(["Izzet"]): "Izzet",
    frozenset(["Orzhov"]): "Orzhov",
    frozenset(["Rakdos"]): "Rakdos",
    frozenset(["Selesnya"]): "Selesnya",
    frozenset(["Simic"]): "Simic",
    frozenset(["Abzan"]): "Abzan",
    frozenset(["Bant"]): "Bant",
    frozenset(["Esper"]): "Esper",
    frozenset(["Grixis"]): "Grixis",
    frozenset(["Jeskai"]): "Jeskai",
    frozenset(["Jund"]): "Jund",
    frozenset(["Mardu"]): "Mardu",
    frozenset(["Naya"]): "Naya",
    frozenset(["Sultai"]): "Sultai",
    frozenset(["Temur"]): "Temur",
    frozenset(["Glint"]): "Glint",
    frozenset(["Dune"]): "Dune",
    frozenset(["Ink"]): "Ink",
    frozenset(["Witch"]): "Witch",
    frozenset(["Yore"]): "Yore",
    frozenset(["Esper"]): "Esper",
    frozenset(["mono-red"]): "Mono-Red",
    frozenset(["mono-white"]): "Mono-White",
    frozenset(["mono-blue"]): "Mono-Blue",
    frozenset(["mono-black"]): "Mono-Black",
    frozenset(["mono-green"]): "Mono-Green",
    frozenset(["all colors"]): "Five Color",
    frozenset(["Colorless"]): "Colorless",
    frozenset(["no data"]): "No Data",
}

# Results dictionary
results = defaultdict(lambda: {"wins": 0, "losses": 0})

# Process each game
for game in games:
    colors = frozenset(game["opponent_colors"])  # Make color order irrelevant
    group = color_groups.get(colors, "Unknown")  # Group by color pair
    if group != "Unknown":
        if game["result"] == "win":
            results[group]["wins"] += 1
        elif game["result"] == "loss":
            results[group]["losses"] += 1
      #  print(f"Processed game: {game}, Group: {group}")
   # else:
   #     print(f"Unknown game encountered: {game}")# Collect results for sorting
win_percentages = []

# Calculate win ratios and prepare for sorting
for group, record in results.items():
    wins = record["wins"]
    losses = record["losses"]
    total_games = wins + losses
    win_ratio = (wins / total_games) * 100 if total_games > 0 else 0
    win_percentages.append((group, wins, losses, win_ratio))

# Sort by win ratio (lowest to highest)
win_percentages.sort(key=lambda x: x[3])  # Sort by win_ratio

# Calculate overall win/loss ratio
total_wins = sum(record["wins"] for record in results.values())
total_losses = sum(record["losses"] for record in results.values())
overall_win_ratio = (total_wins / (total_wins + total_losses) * 100) if (total_wins + total_losses) > 0 else 0

# Save overall win ratio to tracker.txt on line 1
with open(tracker_file_path, 'r+') as file:
    lines = file.readlines()
    if lines:  # If there is any line, clear line 1
        lines[0] = f"Ranked W/L: {total_wins}/{total_losses} ({overall_win_ratio:.2f}%)\n"  # Replace line 1
    else:  # If the file is empty, just add the new line
        lines.append(f"Ranked W/L: {total_wins}/{total_losses} ({overall_win_ratio:.2f}%)\n")
    file.seek(0)  # Move to the beginning of the file
    file.writelines(lines)  # Write the modified lines back to the file
    file.truncate()  # Remove any leftover content

# Create Data directory if it doesn't exist
data_dir = '/home/echo/projects/mtg/mtga_standard_r/Data'
os.makedirs(data_dir, exist_ok=True)

# Prepare DTG filename
dtg_filename = datetime.now().strftime('%d%b%Y%H%M') + '.md'
dtg_file_path = os.path.join(data_dir, dtg_filename)

# Save printed sorted results to a Markdown file
with open(dtg_file_path, 'w') as md_file:
    # Print overall Ranked W/L information at the top of the Markdown file
    md_file.write(f"Ranked W/L: {total_wins}/{total_losses} ({overall_win_ratio:.2f}%)\n\n")
    
    for group, wins, losses, win_ratio in win_percentages:
        md_file.write(f"{group} (Wins: {wins}, Losses: {losses}, Win Ratio: {wins}-{losses} ({win_ratio:.2f}%))\n")

# Print overall Ranked W/L information at the top of the console output
print(f"Ranked W/L: {total_wins}/{total_losses} ({overall_win_ratio:.2f}%)\n")

# Print sorted results to console
for group, wins, losses, win_ratio in win_percentages:
    print(f"{group} (Wins: {wins}, Losses: {losses}, Win Ratio: {wins}-{losses} ({win_ratio:.2f}%))")

