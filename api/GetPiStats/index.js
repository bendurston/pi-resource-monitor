const { CosmosClient } = require('@azure/cosmos');

const endpoint = process.env.endpoint;
const key = process.env.key;
const client = new CosmosClient({ endpoint, key });

module.exports = async function (context, req){
    const piId = (req.query.PiId || (req.body && req.body.PiId))
    const { status, body } = await main(piId);
    context.res = {
        status: status,
        body: body
    }
}


async function main (piId) {
    const { database } = await client.databases.createIfNotExists({ id: "Raspberry Pi's" })
    const { container } = await database.containers.createIfNotExists({ id: "Pi" })
    const { resources } = await container.items.readAll().fetchAll()
    for (const resource of resources) {
        if(resource.id == piId){
            return{
                status: 200,
                body: {
                    "time": resource.time, 
                    "cpu": resource.cpu,
                    "ram": resource.ram,
                    "temp": resource.temp
                }   
            };
        }
    }
    return{
        status: 400,
        body: "No resource found for specified Pi Id."
    };
};

