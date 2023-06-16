# Update crate versions if changes were made
if [ "$first_push" = true ]; then
    echo "First push to origin. Bumping versions for all crates."
    cargo update --workspace

    # Increase the patch version for each crate with changes
    for crate in "utils" "common" "macros"; do
        if git diff --name-only --diff-filter=ACMRTUXB "origin/master" -- "${crate}/"; then
            crate_version=$(awk -F'"' '/version =/{print $2}' "${crate}/Cargo.toml")
            major="${crate_version%%.*}"
            minor="${crate_version#*.}"
            patch="${minor#*.}"
            new_patch=$((patch + 1))
            new_version="${major}.${minor}.${new_patch}"
            sed -i "s/version = \"${crate_version}\"/version = \"${new_version}\"/" "${crate}/Cargo.toml"
            echo "Bumped ${crate} version to ${new_version}"
        fi
    done

    # Increase the patch version for the workspace if it has changes
    if git diff --name-only --diff-filter=ACMRTUXB "origin/master" -- .; then
        workspace_version=$(awk -F'"' '/version =/{print $2}' Cargo.toml)
        major="${workspace_version%%.*}"
        minor="${workspace_version#*.}"
        patch="${minor#*.}"
        new_patch=$((patch + 1))
        new_version="${major}.${minor}.${new_patch}"
        sed -i "s/version = \"${workspace_version}\"/version = \"${new_version}\"/" Cargo.toml
        echo "Bumped workspace version to ${new_version}"
    fi
else
    echo "Checking for version updates in changed crates..."
    master_version=$(git show "origin/master:./Cargo.toml" | awk -F'"' '/version =/{print $2; exit}')

    # Compare and update versions for changed crates
    for crate in "utils" "common" "macros"; do
        if git diff --name-only --diff-filter=ACMRTUXB "origin/master" -- "${crate}/"; then
            crate_version=$(awk -F'"' '/version =/{print $2}' "${crate}/Cargo.toml")
            version_compare "$master_version" "$crate_version"
            case $? in
                1)
                    major="${crate_version%%.*}"
                    minor="${crate_version#*.}"
                    patch="${minor#*.}"
                    new_patch=$((patch + 1))
                    new_version="${major}.${minor}.${new_patch}"
                    sed -i "s/version = \"${crate_version}\"/version = \"${new_version}\"/" "${crate}/Cargo.toml"
                    echo "Bumped ${crate} version to ${new_version}"
                    ;;
                2)
                    echo "Cannot decrease the version of ${crate}. Please fix the version in ${crate}/Cargo.toml."
                    exit 1
                    ;;
                *)
                    echo "No version change required for ${crate}"
                    ;;
            esac
        fi
    done

    # Compare and update version for the workspace if it has changes
    if git diff --name-only --diff-filter=ACMRTUXB "origin/master" -- .; then
        workspace_version=$(awk -F'"' '/version =/{print $2}' Cargo.toml)
        version_compare "$master_version" "$workspace_version"
        case $? in
            1)
                major="${workspace_version%%.*}"
                minor="${workspace_version#*.}"
                patch="${minor#*.}"
                new_patch=$((patch + 1))
                new_version="${major}.${minor}.${new_patch}"
                sed -i "s/version = \"${workspace_version}\"/version = \"${new_version}\"/" Cargo.toml
                echo "Bumped workspace version to ${new_version}"
                ;;
            2)
                echo "Cannot decrease the version of the workspace. Please fix the version in Cargo.toml."
                exit 1
                ;;
            *)
                echo "No version change required for the workspace"
                ;;
        esac
    fi
fi
