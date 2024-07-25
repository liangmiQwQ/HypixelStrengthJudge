import { Button, Theme } from "@radix-ui/themes";
import "../global.css"
import '@radix-ui/themes/styles.css';
import { useRef } from "react";

export default function index() {
  const dragRef = useRef(null)

  return (
    <Theme appearance="light" className="rounded-lg overflow-hidden">
      <div className="w-full h-screen rounded-md bg-white overflow-hidden">
        <div className="bg-accent-700 h-12 w-full flex justify-between items-center">
          <div>

          </div>
          <span className="text-white font-semibold text-2xl cursor-default" ref={dragRef}>
            我是傻逼
          </span>
          <div>
          </div>
        </div>
      </div>
    </Theme>
  )
}

