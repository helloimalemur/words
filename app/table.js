import { open } from '@tauri-apps/plugin-dialog';
import {invoke} from "@tauri-apps/api/tauri";
import {useState} from "react";


let files = [
  // { index: 0, name: 'Lindsay Walton', title: 'Front-end Developer', path: '/home/foxx/test', word_count: '0' },
  // More people...
]

export default function Table() {
  const [removal, setRemoval] = useState(0);
  const [addition, setAddition] = useState("");

  const printall = async () => {
    invoke('printall').then(r => {})
  }

  const remove_entry = async function (index) {
    console.log("Removing: " + removal)
    setRemoval(index)
    invoke('remove_entry', {index: removal}).then(r => {})
  }

  const add_entry = async function (path) {
        // const file = await open({
        //   multiple: false,
        //   directory: false,
        // });
        console.log("Adding: " + path)
        // invoke('add_entry', {index: removal}).then(r => {})
  }

  // const get_all_entries = async function (index) {
  //   console.log("Updating..")
  //   invoke('get_all_entries').then(r => {})
  //   setAllentries(index)
  // }

  return (
    <div className="px-4 sm:px-6 lg:px-8">
      <div className="sm:flex sm:items-center">
        <div className="sm:flex-auto">
          <h1 className="text-base font-semibold leading-6 text-gray-900">Files</h1>
          {/*<p className="mt-2 text-sm text-gray-700">*/}
          {/*  A list of all the users in your account including their name, title, email and role.*/}
          {/*</p>*/}
        </div>
          <div className="mt-4 sm:ml-16 sm:mt-0 sm:flex-none">
          {/*<Greet/>*/}
          Add file
          <input
              type="file"
              // id="filepicker"
              onChange={(e) => {
                  const file = e.target.files[0];
                  setAddition(file.webkitRelativePath)
              }}
              className="block rounded-md bg-indigo-600 px-3 py-2 text-center text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
          />
          <button
              onClick={() => {add_entry(addition)}}
              type="button"
              className="block rounded-md bg-indigo-600 px-3 py-2 text-center text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
          >
          </button>
          </div>
      </div>
        <div className="mt-8 flow-root">
            <div className="-mx-4 -my-2 overflow-x-auto sm:-mx-6 lg:-mx-8">
                <div className="inline-block min-w-full py-2 align-middle sm:px-6 lg:px-8">
                    <table className="min-w-full divide-y divide-gray-300">
                        <thead>
                        <tr>
                            <th scope="col" className="py-3.5 pl-4 pr-3 text-left text-sm font-semibold text-gray-900 sm:pl-0">
                    Name
                  </th>
                  <th scope="col" className="px-3 py-3.5 text-left text-sm font-semibold text-gray-900">
                    Title
                  </th>
                  <th scope="col" className="px-3 py-3.5 text-left text-sm font-semibold text-gray-900">
                    Path
                  </th>
                  <th scope="col" className="px-3 py-3.5 text-left text-sm font-semibold text-gray-900">
                    Word Count
                  </th>
                  <th scope="col" className="relative py-3.5 pl-3 pr-4 sm:pr-0">
                    <span className="sr-only">Edit</span>
                  </th>
                </tr>
              </thead>
              <tbody className="divide-y divide-gray-200">
                {files.map((file) => (
                  <tr key={file.path}>
                    <td className="whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-gray-900 sm:pl-0">
                      {file.name}
                    </td>
                    <td className="whitespace-nowrap px-3 py-4 text-sm text-gray-500">{file.title}</td>
                    <td className="whitespace-nowrap px-3 py-4 text-sm text-gray-500">{file.path}</td>
                    <td className="whitespace-nowrap px-3 py-4 text-sm text-gray-500">{file.word_count}</td>
                    <td className="relative whitespace-nowrap py-4 pl-3 pr-4 text-right text-sm font-medium sm:pr-0">
                      <a
                        onClick={() => {
                          remove_entry(file.index).then(r => r)
                        }}
                        // onchange={(e) => {
                        //   console.log("Removing: " + e.target.value)
                        //   setRemoval(e.target.value)
                        // }}
                        href="#"
                        className="text-indigo-600 hover:text-indigo-900"
                      >
                        Edit<span className="sr-only">, {file.name}</span>
                      </a>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>
  )
}
