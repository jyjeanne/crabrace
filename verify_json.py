#!/usr/bin/env python3
"""
Verify that our JSON configuration files are valid and match the expected schema.
This is a workaround to test our JSON configs while the Rust build is broken.
"""

import json
import sys
from pathlib import Path

def validate_provider(provider_data, filename):
    """Validate a provider configuration."""
    errors = []

    # Required fields
    required_fields = ['name', 'id', 'type', 'models']
    for field in required_fields:
        if field not in provider_data:
            errors.append(f"Missing required field: {field}")

    # Optional fields that should be present for API compatibility
    expected_optional = ['api_key', 'api_endpoint', 'default_large_model_id',
                        'default_small_model_id', 'default_headers']
    for field in expected_optional:
        if field not in provider_data:
            print(f"  ⚠️  Missing optional field: {field}")

    # Validate models
    if 'models' in provider_data:
        for i, model in enumerate(provider_data['models']):
            model_errors = validate_model(model, i)
            errors.extend([f"Model {i}: {e}" for e in model_errors])

    return errors

def validate_model(model_data, index):
    """Validate a model configuration."""
    errors = []

    # Required fields
    required = ['id', 'name', 'cost_per_1m_in', 'cost_per_1m_out',
                'context_window', 'default_max_tokens']
    for field in required:
        if field not in model_data:
            errors.append(f"Missing required field: {field}")

    # Check for old field names that should be updated
    old_fields = {
        'cost_per_1m_input': 'cost_per_1m_in',
        'cost_per_1m_output': 'cost_per_1m_out',
        'supports_images': 'supports_attachments',
        'base_url': 'api_endpoint'
    }

    for old, new in old_fields.items():
        if old in model_data:
            errors.append(f"Old field name '{old}' should be '{new}'")

    # Boolean fields should default to false
    bool_fields = ['can_reason', 'has_reasoning_efforts', 'supports_attachments']
    for field in bool_fields:
        if field in model_data and not isinstance(model_data[field], bool):
            errors.append(f"Field '{field}' should be boolean")

    return errors

def main():
    """Main validation function."""
    configs_dir = Path('src/providers/configs')

    if not configs_dir.exists():
        print(f"[ERROR] Directory not found: {configs_dir}")
        return 1

    json_files = list(configs_dir.glob('*.json'))

    if not json_files:
        print(f"[ERROR] No JSON files found in {configs_dir}")
        return 1

    print(f"[*] Validating {len(json_files)} provider configuration(s)...\n")

    total_errors = 0

    for json_file in sorted(json_files):
        print(f"[FILE] {json_file.name}")

        try:
            with open(json_file, 'r', encoding='utf-8') as f:
                data = json.load(f)

            errors = validate_provider(data, json_file.name)

            if errors:
                for error in errors:
                    print(f"  [ERROR] {error}")
                total_errors += len(errors)
            else:
                model_count = len(data.get('models', []))
                provider_type = data.get('type', 'unknown')
                print(f"  [OK] Valid! ({model_count} models, type: {provider_type})")

                # Show key info
                if 'default_large_model_id' in data:
                    print(f"     Large: {data['default_large_model_id']}")
                if 'default_small_model_id' in data:
                    print(f"     Small: {data['default_small_model_id']}")

            print()

        except json.JSONDecodeError as e:
            print(f"  [ERROR] JSON parse error: {e}")
            total_errors += 1
            print()
        except Exception as e:
            print(f"  [ERROR] Error: {e}")
            total_errors += 1
            print()

    if total_errors == 0:
        print("[SUCCESS] All provider configurations are valid!")
        print("\n[SUMMARY]")
        total_models = 0
        for json_file in sorted(json_files):
            with open(json_file, 'r', encoding='utf-8') as f:
                data = json.load(f)
                model_count = len(data.get('models', []))
                total_models += model_count
                print(f"  - {data['name']}: {model_count} models")
        print(f"\nTotal: {len(json_files)} providers, {total_models} models")
        return 0
    else:
        print(f"[FAILED] Found {total_errors} error(s)")
        return 1

if __name__ == '__main__':
    sys.exit(main())
