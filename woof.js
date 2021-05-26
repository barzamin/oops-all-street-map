// woofy
console.log("map setup");
const globalmap = L.map('globalmap').setView([60.8002891326115,90.87890625000001], 5);
const googleHybrid = L.tileLayer('http://{s}.google.com/vt/lyrs=s,h&x={x}&y={y}&z={z}',{
    maxZoom: 20,
    subdomains:['mt0','mt1','mt2','mt3']
});
googleHybrid.addTo(globalmap);
for (const [wayid, way] of window.ways) {
    for (const pole of way.poles) {
        const p = {lat: pole.decimicro_lat * 1e-7, lng: pole.decimicro_lon * 1e-7};
        const mapsurl = `https://www.google.com/maps/place/${p.lat},${p.lng}`;
        const o = L.circle(p, {
            color: 'red',
            radius: 50,
        })
        .bindPopup(`<a href="#way${way.way.id}">way ${way.way.id}</a> | <a href=${mapsurl} target="_blank">gmaps</a>`)
        .addTo(globalmap);
    }
}
globalmap.on('moveend', (e) => {
    const c = e.target.getCenter();
    document.getElementById('latlon').innerHTML = `${c.lat},${c.lng}`;
});
