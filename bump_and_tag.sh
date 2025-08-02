
set -euo pipefail

if [ $# != 1 ]
then
	echo "need 1 arguments: new versions"
	exit 1
fi

new_version=$1
echo "bumping version from to ${new_version}"


echo "  - updating Cargo.toml version"
if [[ $(git status --porcelain 2> /dev/null | grep -v "??" | wc -l)  != "0" ]]
then
	echo "git workspace is dirty. Please commit your changes before tagging a version"
	exit 1
fi

sed -i -E "s/^version = \"[^\"]*\"$/version = \"${new_version}\"/" Cargo.toml
echo "  - committing Cargo.toml version change"
git add Cargo.toml
git commit -m "bumping version in Cargo.toml to ${new_version}"



echo "  - tagging"
# echo -e "$new_version\n\nCurrent roadmap:\n" | cat - docs/roadmap.md |git tag $new_version -a --file=-
git tag "$new_version" -m "$(git show HEAD~1 --pretty="%s" |head -n 1)"

echo -e "version bumping done\n"
echo "optionally, do the next command to edit the tag with the highlights of this version:"
echo "git tag -a -f ${new_version}"





