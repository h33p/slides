#!/bin/sh

gamma="0.45454545"

for f in $1/*.exr; do
	# Handle cases where this resolves into glob itself
	if [ ! -f "$f" ]; then
		break
	fi
	name="$(basename "$f")"
	name="${name%.exr}"
	ffmpeg -i "$f" -vf "lutrgb=r=gammaval(${gamma}):g=gammaval(${gamma}):b=gammaval(${gamma})" -q:v 1 -frames:v 1 "$2/$name.jpg"
done
