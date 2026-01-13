#!/usr/bin/env bash
set -euo pipefail

# sync_versions.sh
# Synchronizes all cortenforge-* internal dependency versions with the workspace version
# from the root Cargo.toml.

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to extract workspace version from root Cargo.toml
get_workspace_version() {
    local version=$(grep '^version = ' Cargo.toml | head -1 | sed -E 's/version = "(.*)"/\1/')
    if [[ -z "$version" ]]; then
        echo -e "${RED}Error: Could not find workspace version in Cargo.toml${NC}" >&2
        exit 1
    fi
    echo "$version"
}

# Function to find all Cargo.toml files (excluding target/)
find_cargo_tomls() {
    find . -name "Cargo.toml" -not -path "*/target/*" -not -path "*/.git/*"
}

# Function to update cortenforge-* dependencies in a file
update_dependencies() {
    local file="$1"
    local target_version="$2"
    local temp_file="${file}.tmp"

    # Use sed to replace cortenforge-* dependency versions
    # Matches patterns like: cortenforge-foo", version = "0.5.1"
    sed -E "s/(cortenforge-[a-z-]+\", version = \")([0-9]+\.[0-9]+\.[0-9]+)(\")/\1${target_version}\3/g" "$file" > "$temp_file"

    # Check if there were any changes
    if ! diff -q "$file" "$temp_file" > /dev/null 2>&1; then
        mv "$temp_file" "$file"
        echo "$file"
    else
        rm "$temp_file"
    fi
}

# Main script
main() {
    echo "🔍 Checking workspace version..."
    WORKSPACE_VERSION=$(get_workspace_version)
    echo -e "${GREEN}Workspace version: ${WORKSPACE_VERSION}${NC}"
    echo ""

    echo "🔍 Scanning for version mismatches in cortenforge-* dependencies..."
    echo ""

    # Find all Cargo.toml files
    CARGO_TOMLS=$(find_cargo_tomls)

    # Track if any changes were made
    CHANGES_MADE=false
    CHANGED_FILES=()

    # Check for mismatches and collect diffs
    for toml_file in $CARGO_TOMLS; do
        # Skip the root Cargo.toml for dependency scanning (but we'll check it for package version)
        if [[ "$toml_file" == "./Cargo.toml" ]]; then
            continue
        fi

        # Check if this file has any cortenforge-* dependencies with wrong versions
        MISMATCHES=$(grep -E "cortenforge-[a-z-]+\", version = \"[0-9]+\.[0-9]+\.[0-9]+\"" "$toml_file" 2>/dev/null | grep -v "version = \"${WORKSPACE_VERSION}\"" || true)

        if [[ -n "$MISMATCHES" ]]; then
            echo -e "${YELLOW}Found mismatches in ${toml_file}:${NC}"
            echo "$MISMATCHES"
            echo ""
            CHANGES_MADE=true
        fi
    done

    if [[ "$CHANGES_MADE" == false ]]; then
        echo -e "${GREEN}✓ All cortenforge-* dependencies are already synchronized to ${WORKSPACE_VERSION}${NC}"
        echo -e "${GREEN}✓ No changes needed${NC}"
        exit 0
    fi

    # Show what will be changed
    echo -e "${YELLOW}The following files will be updated to use version ${WORKSPACE_VERSION}:${NC}"
    echo ""

    # Dry-run mode by default
    DRY_RUN=true
    if [[ "${1:-}" == "--apply" ]]; then
        DRY_RUN=false
    fi

    if [[ "$DRY_RUN" == true ]]; then
        echo -e "${YELLOW}Running in DRY-RUN mode. Use --apply to make changes.${NC}"
        echo ""
    fi

    # Update dependencies
    for toml_file in $CARGO_TOMLS; do
        if [[ "$toml_file" == "./Cargo.toml" ]]; then
            continue
        fi

        # Check if file has cortenforge dependencies that need updating
        if grep -qE "cortenforge-[a-z-]+\", version = \"[0-9]+\.[0-9]+\.[0-9]+\"" "$toml_file" 2>/dev/null; then
            if grep -qE "cortenforge-[a-z-]+\", version = \"[0-9]+\.[0-9]+\.[0-9]+\"" "$toml_file" 2>/dev/null | grep -qv "version = \"${WORKSPACE_VERSION}\""; then
                if [[ "$DRY_RUN" == false ]]; then
                    UPDATED_FILE=$(update_dependencies "$toml_file" "$WORKSPACE_VERSION")
                    if [[ -n "$UPDATED_FILE" ]]; then
                        CHANGED_FILES+=("$UPDATED_FILE")
                        echo -e "${GREEN}✓ Updated: ${UPDATED_FILE}${NC}"
                    fi
                else
                    echo -e "  Would update: ${toml_file}"
                fi
            fi
        fi
    done

    echo ""

    if [[ "$DRY_RUN" == true ]]; then
        echo -e "${YELLOW}To apply these changes, run:${NC}"
        echo -e "${YELLOW}  ./scripts/sync_versions.sh --apply${NC}"
        echo ""
        echo -e "${YELLOW}Then verify with:${NC}"
        echo -e "${YELLOW}  cargo check --workspace${NC}"
    else
        echo -e "${GREEN}✓ Version synchronization complete!${NC}"
        echo -e "${GREEN}Changed ${#CHANGED_FILES[@]} file(s)${NC}"
        echo ""
        echo "Next steps:"
        echo "  1. Review changes: git diff"
        echo "  2. Verify build: cargo check --workspace"
        echo "  3. Commit: git commit -am 'sync: Update internal dependency versions to ${WORKSPACE_VERSION}'"
    fi
}

main "$@"
