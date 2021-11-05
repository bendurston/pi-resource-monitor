const { CosmosClient, ConflictResolutionMode } = require('@azure/cosmos');

const endpoint = process.env.endpoint;
const key = process.env.key;
const client = new CosmosClient({ endpoint, key });

module.exports = async function (context, req) {
    const piId = (req.query.piId || (req.body && req.body.piId))
    const temp =  (req.query.temp || (req.body && req.body.temp))
    const cpu = (req.query.cpu || (req.body && req.body.cpu))
    const ram = (req.query.ram || (req.body && req.body.ram))
    const time = new Date().toLocaleString()

    const cpu_parsed = parseCPUResource(cpu)
    const ram_parsed = parseRAMResource(ram)

    const { status, body } = await main(piId, time, cpu_parsed, ram_parsed, temp)

    context.res = {
        status: status,
        body: body
    }
};

async function main(piId, time, cpu, ram, temp) {
    const { database } = await client.databases.createIfNotExists({ id: "Raspberry Pi's" })
    const { container } = await database.containers.createIfNotExists({ id: "Pi" })
    const { resources } = await container.items.readAll().fetchAll()
    
    const resource = await getResource(resources, piId)
    if (resource == null) {
        return {
            status: 400,
            body: "There is no Pi with the specified ID."
        }
    };

    const appendedResource = setStatsToExistingResource(resource, time, cpu, ram, temp)

    const updatedResource = {
        "id": `${piId}`,
        "time": appendedResource.time,
        "cpu": appendedResource.cpu,
        "ram": appendedResource.ram,
        "temp": appendedResource.temp
    }

    await container.items.upsert(updatedResource);
    return {
        status: 200,
        body: "Success!"
    };
};

function parseCPUResource(cpu) {
    cpu_arr = cpu.split(" ")
    let cpu_arr_parsed = cpu_arr.filter(item => item != '')
    let total = cpu_arr_parsed[7]
    let taken = Number((100 - total).toFixed(2))
    return taken
}

function parseRAMResource(ram) {
    ram_arr = ram.split(" ")
    let ram_arr_parsed = ram_arr.filter(item => item != '')
    let total = ram_arr_parsed[3]
    let free = ram_arr_parsed[5]
    let taken = Number(((1 - (free / total)) * 100).toFixed(2))
    return taken
}

async function getResource(resources, piId){
    for (const resource of resources) {
        if(resource.id == piId){
            return resource
        } 
    }
    return null
};

function setStatsToExistingResource(resource, time, cpu, ram, temp) {
    if (resource.time.length >= 100) resource.time.shift()
    resource.time.push(time)

    if (resource.cpu.length >= 100) resource.cpu.shift()
    resource.cpu.push(cpu)

    if (resource.ram.length >= 100) resource.ram.shift()
    resource.ram.push(ram)

    if (resource.temp.length >= 100) resource.temp.shift()
    resource.temp.push(temp)

    return resource
};

