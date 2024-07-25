import { Select } from "@radix-ui/themes";

export interface settingOptionProps {
  tipsText: string;
  optionType: "select" | "input" | "switch";
  selectValue?: { name: string, id: string }[]
  placeholder?: string;
  defaultChecked?: boolean;
  defaultValue?: string;
  onValueChange: (value: any) => any
}

export default function SettingOption({ tipsText, optionType, selectValue, defaultValue }: settingOptionProps) {
  return (
    <div className='w-full flex justify-between items-center'>
      <span>
        {tipsText}
      </span>
      {optionType === "select" && (
        <Select.Root defaultValue={defaultValue}>
          <Select.Trigger />
          <Select.Content>
            {(selectValue as { name: string, id: string }[]).map((item, index) => {
              return (<Select.Item key={item.id + item.name + index}
                value={(selectValue as { name: string, id: string }[])[index].id}>{(selectValue as any)[index].name}</Select.Item>)
            })}
          </Select.Content>
        </Select.Root>
      )}
    </div>
  )
}

