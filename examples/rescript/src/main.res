type SendEmailRequest = Types.SendEmailRequest

let filePath = "/tmp/message"

let writeToFile = () => {
  let message = {
    to: "typical@example.com",
    subject: "I love Typical!",
    body: "It makes serialization easy and safe.",
  }

  let arrayBuffer = SendEmailRequest.serialize(message)
  writeFileSync(filePath, Buffer.from(arrayBuffer))
}

let readFromFile = () => {
  let fileContents = readFileSync(filePath)

  let message = SendEmailRequest.deserialize(
    new DataView(
      fileContents.buffer,
      fileContents.byteOffset,
      fileContents.byteLength,
    ),
  );

  Js.log2("to:", message.to)
  Js.log2("subject:", message.subject)
  Js.log2("body:", message.body)
}

writeToFile()
readFromFile()
unlinkSync(filePath)
