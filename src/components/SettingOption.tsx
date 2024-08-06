import { Select } from "@radix-ui/themes";
import { debounce } from "lodash";
import { useCallback } from "react";

export interface settingOptionProps {
  tipsText: string;
  optionType: "select" | "input" | "switch";
  selectValue?: { name: string; id: string }[];
  placeholder?: string;
  defaultChecked?: boolean;
  defaultValue?: string;
  onValueChange?: (value: any) => any;
}

export default function SettingOption({
  placeholder,
  tipsText,
  optionType,
  selectValue,
  defaultValue,
  onValueChange,
}: settingOptionProps) {
  const handleChange = (value: any) => {
    onValueChange?.(value);
  };

  const debouncedHandleChange = useCallback(debounce(handleChange, 300), [handleChange]);

  return (
    <div
      className={
        "w-full flex justify-between items-center" + (optionType === "input" ? " my-1" : "")
      }
    >
      <span>{tipsText}</span>
      {optionType === "select" && (
        <Select.Root defaultValue={defaultValue} onValueChange={handleChange}>
          <Select.Trigger />
          <Select.Content>
            {(selectValue as { name: string; id: string }[]).map((item, index) => {
              return (
                <Select.Item
                  key={item.id + item.name + index}
                  value={(selectValue as { name: string; id: string }[])[index].id}
                >
                  {(selectValue as any)[index].name}
                </Select.Item>
              );
            })}
          </Select.Content>
        </Select.Root>
      )}
      {optionType === "input" && (
        <input
          defaultValue={defaultValue}
          onChange={e => debouncedHandleChange(e.target.value)}
          placeholder={placeholder}
          className="flex-1 max-w-40 border-b-slate-600 focus:border-b-black border-b-[1px] outline-none"
        />
      )}
    </div>
  );
}
